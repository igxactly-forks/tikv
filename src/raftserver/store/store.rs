#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::sync::{Arc, Mutex, RwLock};
use std::option::Option;
use std::collections::{HashMap, HashSet};

use rocksdb::DB;
use mio::{self, EventLoop};

use proto::raft_serverpb::{RaftMessage, StoreIdent};
use raftserver::{Result, other};
use super::{Sender, Msg};
use super::keys;
use super::engine::Retriever;
use super::config::Config;
use super::peer::Peer;
use super::transport::Transport;

pub struct Store<T: Transport> {
    cfg: Config,
    ident: StoreIdent,
    engine: Arc<DB>,
    sender: Sender,
    event_loop: Option<EventLoop<Store<T>>>,

    peers: HashMap<u64, Peer>,
    pending_raft_groups: HashSet<u64>,

    trans: Arc<RwLock<T>>,
}

impl<T: Transport> Store<T> {
    pub fn new(cfg: Config, engine: Arc<DB>, trans: Arc<RwLock<T>>) -> Result<Store<T>> {
        let ident: StoreIdent = try!(load_store_ident(engine.as_ref()).and_then(|res| {
            match res {
                None => Err(other("store must be bootstrapped first")),
                Some(ident) => Ok(ident),
            }
        }));

        let event_loop = try!(EventLoop::new());
        let sender = Sender::new(event_loop.channel());

        Ok(Store {
            cfg: cfg,
            ident: ident,
            engine: engine,
            sender: sender,
            event_loop: Some(event_loop),
            peers: HashMap::new(),
            pending_raft_groups: HashSet::new(),
            trans: trans,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut event_loop = self.event_loop.take().unwrap();
        self.register_raft_base_tick(&mut event_loop);
        try!(event_loop.run(self));
        Ok(())
    }

    pub fn get_sender(&self) -> Sender {
        self.sender.clone()
    }

    pub fn get_engine(&self) -> Arc<DB> {
        self.engine.clone()
    }

    pub fn get_ident(&self) -> &StoreIdent {
        &self.ident
    }

    pub fn get_store_id(&self) -> u64 {
        self.ident.get_store_id()
    }

    pub fn get_config(&self) -> &Config {
        &self.cfg
    }

    fn register_raft_base_tick(&self, event_loop: &mut EventLoop<Self>) {
        // If we register raft base tick failed, the whole raft can't run correctly,
        // TODO: shutdown the store?
        let _ = register_timer(event_loop,
                               Msg::RaftBaseTick,
                               self.cfg.raft_base_tick_interval)
                    .map_err(|e| {
                        error!("register raft base tick err: {:?}", e);
                    });
    }

    fn handle_raft_base_tick(&mut self, event_loop: &mut EventLoop<Self>) {
        for (region_id, peer) in self.peers.iter_mut() {
            peer.raft_group.tick();
            self.pending_raft_groups.insert(*region_id);
        }

        self.register_raft_base_tick(event_loop);
    }

    fn handle_raft_message(&mut self, msg: Mutex<RaftMessage>) -> Result<()> {
        let msg = msg.lock().unwrap();

        let region_id = msg.get_region_id();
        let from_peer = msg.get_from_peer();
        let to_peer = msg.get_to_peer();
        debug!("handle raft message for region {}, from {} to {}",
               region_id,
               from_peer.get_peer_id(),
               to_peer.get_peer_id());

        {
            let mut trans = self.trans.write().unwrap();
            trans.cache_peer(from_peer.get_peer_id(), from_peer.clone());
            trans.cache_peer(to_peer.get_peer_id(), to_peer.clone());
        }

        if !self.peers.contains_key(&region_id) {
            let peer = try!(Peer::replicate(self, region_id, to_peer.get_peer_id()));
            self.peers.insert(region_id, peer);
        }

        let mut peer = self.peers.get_mut(&region_id).unwrap();

        try!(peer.raft_group.step(msg.get_message().clone()));

        // Add in pending raft group for later handing ready.
        self.pending_raft_groups.insert(region_id);

        Ok(())
    }

    fn handle_raft_ready(&mut self) -> Result<()> {
        let ids = self.pending_raft_groups.drain();

        for region_id in ids {
            if let Some(peer) = self.peers.get_mut(&region_id) {
                try!(peer.handle_raft_ready(&self.trans).map_err(|e| {
                    // TODO: should we panic here or shutdown the store?
                    error!("handle raft ready err: {:?}", e);
                    e
                }));
            }
        }

        Ok(())
    }
}

fn load_store_ident<T: Retriever>(r: &T) -> Result<Option<StoreIdent>> {
    let ident = try!(r.get_msg::<StoreIdent>(&keys::store_ident_key()));

    Ok(ident)
}

fn register_timer<T: Transport>(event_loop: &mut EventLoop<Store<T>>,
                                msg: Msg,
                                delay: u64)
                                -> Result<mio::Timeout> {
    // TODO: now mio TimerError doesn't implement Error trait,
    // so we can't use `try!` directly.
    event_loop.timeout_ms(msg, delay).map_err(|e| other(format!("register timer err: {:?}", e)))
}

impl<T: Transport> mio::Handler for Store<T> {
    type Timeout = Msg;
    type Message = Msg;

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Msg) {
        match msg {
            Msg::RaftMessage(data) => {
                self.handle_raft_message(data).map_err(|e| {
                    error!("handle raft message err: {:?}", e);
                });
            }
            _ => panic!("invalid notify msg type {:?}", msg),
        }
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Msg) {
        match timeout {
            Msg::RaftBaseTick => self.handle_raft_base_tick(event_loop),
            _ => panic!("invalid timeout msg type {:?}", timeout),
        }
    }

    fn tick(&mut self, event_loop: &mut EventLoop<Self>) {
        // We handle raft ready in event loop.
        self.handle_raft_ready().map_err(|e| {
            // TODO: should we panic here or shutdown the store?
            error!("handle raft ready err: {:?}", e);
        });
    }
}
