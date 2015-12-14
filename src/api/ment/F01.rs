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

//get file list
pub struct F01;

impl DataApi for F01 {

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
        let table = db.get_table("file").expect("file table not exists.");
        let c_data = table.count_by_str("{}", "{}").unwrap();
        println!("{}", c_data);
        let mut data = table.find_by_str("{}", "{}", "{}");
        {
            let mut data_obj = data.as_object_mut().unwrap();
            data_obj.insert("count".to_string(), json_i64!(&c_data; "data", "0", "count").to_json());
        }
        Result::Ok(data)
    }

}
