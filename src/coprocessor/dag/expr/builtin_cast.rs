// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

// FIXME(shirly): remove following later
#![allow(dead_code)]

use std::{i64, u64};
use super::{FnCall, StatementContext, Result};
use coprocessor::codec::{Datum, mysql};
use coprocessor::codec::mysql::{Decimal, Time, Duration};
use coprocessor::codec::convert::{convert_int_to_uint, convert_float_to_int, convert_float_to_uint};
use coprocessor::codec::mysql::types;

impl FnCall {
    pub fn cast_int_as_int(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<i64>> {
        self.children[0].eval_int(ctx, row)
    }

    pub fn cast_real_as_int(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<i64>> {
        let val = try!(self.children[0].eval_real(ctx, row));
        if val.is_none() {
            return Ok(None);
        }
        let val = val.unwrap();
        if !mysql::has_unsigned_flag(self.tp.get_flag() as u64) {
            let res = try!(convert_float_to_int(val, i64::MIN, i64::MAX, types::DOUBLE));
            Ok(Some(res))
        } else {
            let uval = try!(convert_float_to_uint(val, u64::MAX, types::DOUBLE));
            Ok(Some(uval as i64))
        }
    }

    pub fn cast_decimal_as_int(&self,
                               ctx: &StatementContext,
                               row: &[Datum])
                               -> Result<Option<i64>> {
        unimplemented!()
    }

    pub fn cast_str_as_int(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<i64>> {
        unimplemented!()
    }

    pub fn cast_time_as_int(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<i64>> {
        unimplemented!()
    }

    pub fn cast_duration_as_int(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<i64>> {
        unimplemented!()
    }

    pub fn cast_int_as_real(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<f64>> {
        let val = try!(self.children[0].eval_int(ctx, row));
        if val.is_none() {
            return Ok(None);
        }
        let val = val.unwrap();
        if !mysql::has_unsigned_flag(self.children[0].get_tp().get_flag() as u64) {
            Ok(Some(val as f64))
        } else {
            let uval = try!(convert_int_to_uint(val, u64::MAX, types::LONG_LONG));
            Ok(Some(uval as f64))
        }
    }

    pub fn cast_real_as_real(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<f64>> {
        unimplemented!()
    }

    pub fn cast_decimal_as_real(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<f64>> {
        unimplemented!()
    }

    pub fn cast_str_as_real(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<f64>> {
        unimplemented!()
    }

    pub fn cast_time_as_real(&self, ctx: &StatementContext, row: &[Datum]) -> Result<Option<f64>> {
        unimplemented!()
    }

    pub fn cast_duration_as_real(&self,
                                 ctx: &StatementContext,
                                 row: &[Datum])
                                 -> Result<Option<f64>> {
        unimplemented!()
    }

    pub fn cast_int_as_decimal(&self,
                               ctx: &StatementContext,
                               row: &[Datum])
                               -> Result<Option<Decimal>> {
        let val = try!(self.children[0].eval_int(ctx, row));
        if val.is_none() {
            return Ok(None);
        }
        let val = val.unwrap();
        let field_type = &self.children[0].get_tp();
        let res = if !mysql::has_unsigned_flag(field_type.get_flag() as u64) {
            Decimal::from(val)
        } else {
            let uval = try!(convert_int_to_uint(val, u64::MAX, types::LONG_LONG));
            Decimal::from(uval)
        };
        let res = try!(res.convert_to(ctx, field_type.get_flen(), field_type.get_decimal()));
        Ok(Some(res))
    }

    pub fn cast_real_as_decimal(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<Decimal>> {
        unimplemented!()
    }

    pub fn cast_decimal_as_decimal(&self,
                                   ctx: &StatementContext,
                                   row: &[Datum])
                                   -> Result<Option<Decimal>> {
        unimplemented!()
    }

    pub fn cast_str_as_decimal(&self,
                               ctx: &StatementContext,
                               row: &[Datum])
                               -> Result<Option<Decimal>> {
        unimplemented!()
    }

    pub fn cast_time_as_decimal(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<Decimal>> {
        unimplemented!()
    }

    pub fn cast_duration_as_decimal(&self,
                                    ctx: &StatementContext,
                                    row: &[Datum])
                                    -> Result<Option<Decimal>> {
        unimplemented!()
    }

    pub fn cast_int_as_str(&self,
                           ctx: &StatementContext,
                           row: &[Datum])
                           -> Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    pub fn cast_real_as_str(&self,
                            ctx: &StatementContext,
                            row: &[Datum])
                            -> Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    pub fn cast_decimal_as_str(&self,
                               ctx: &StatementContext,
                               row: &[Datum])
                               -> Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    pub fn cast_str_as_str(&self,
                           ctx: &StatementContext,
                           row: &[Datum])
                           -> Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    pub fn cast_time_as_strl(&self,
                             ctx: &StatementContext,
                             row: &[Datum])
                             -> Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    pub fn cast_duration_as_str(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<Vec<u8>>> {
        unimplemented!()
    }

    pub fn cast_int_as_time(&self,
                            ctx: &StatementContext,
                            row: &[Datum])
                            -> Result<Option<Vec<Time>>> {
        unimplemented!()
    }

    pub fn cast_real_as_time(&self,
                             ctx: &StatementContext,
                             row: &[Datum])
                             -> Result<Option<Vec<Time>>> {
        unimplemented!()
    }

    pub fn cast_decimal_as_time(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<Vec<Time>>> {
        unimplemented!()
    }

    pub fn cast_str_as_time(&self,
                            ctx: &StatementContext,
                            row: &[Datum])
                            -> Result<Option<Vec<Time>>> {
        unimplemented!()
    }

    pub fn cast_time_as_time(&self,
                             ctx: &StatementContext,
                             row: &[Datum])
                             -> Result<Option<Vec<Time>>> {
        unimplemented!()
    }

    pub fn cast_duration_as_time(&self,
                                 ctx: &StatementContext,
                                 row: &[Datum])
                                 -> Result<Option<Vec<Time>>> {
        unimplemented!()
    }

    pub fn cast_int_as_duration(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<Vec<Duration>>> {
        unimplemented!()
    }

    pub fn cast_real_as_duration(&self,
                                 ctx: &StatementContext,
                                 row: &[Datum])
                                 -> Result<Option<Vec<Duration>>> {
        unimplemented!()
    }

    pub fn cast_decimal_as_duration(&self,
                                    ctx: &StatementContext,
                                    row: &[Datum])
                                    -> Result<Option<Vec<Duration>>> {
        unimplemented!()
    }

    pub fn cast_str_as_duration(&self,
                                ctx: &StatementContext,
                                row: &[Datum])
                                -> Result<Option<Vec<Duration>>> {
        unimplemented!()
    }

    pub fn cast_time_as_duration(&self,
                                 ctx: &StatementContext,
                                 row: &[Datum])
                                 -> Result<Option<Vec<Duration>>> {
        unimplemented!()
    }

    pub fn cast_duration_as_duration(&self,
                                     ctx: &StatementContext,
                                     row: &[Datum])
                                     -> Result<Option<Vec<Duration>>> {
        unimplemented!()
    }
}