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

//user query self info 
pub struct U03;

impl DataApi for U03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> 
    {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> 
    {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> 
    {
        let table = db.get_table("customer").expect("st table not exists.");
        let cond = json_str!(msg; "body", "cond");
        let limit = json_i64!(msg; "body", "limit");
        let offset = json_i64!(msg; "body", "offset");
        let op = format!(r#"
            {{
                "offset":{},
                "limit":{}
            }}
        "#, offset, limit);
        let c_data = try!(table.count_by_str(cond, "{}"));
        let mut data = table.find_by_str(cond, "{}", &op);
        {
            let count = json_i64!(&c_data; "data", "0", "count");
            json_set!(&mut data;"count";count);
        }
        Result::Ok(data)
    }

}
