use super::super::super::util::DigestUtil;
use super::super::super::dc::DataBase;
use super::super::super::dc::MyDbPool;
use super::super::super::cons::CONS;
use super::super::super::cons::ErrCode;

use std::collections::BTreeMap;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate regex;
use self::regex::Regex;

extern crate time;

use super::super::inter::{DataApi};
use super::super::util::{KeyHelper};

/**
 * index page, get the popular's book type.
 */
pub struct GBT01;

impl DataApi for GBT01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> 
    {
        Result::Ok(DigestUtil::empty_key())
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> 
    {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> 
    {
        let table = db.get_table("book_type").expect("st table not exists.");
        Result::Ok(json!("{}"))
    }

}
