use super::super::super::util::DigestUtil;
use super::super::super::dc::DataBase;
use super::super::super::dc::MyDbPool;
use super::super::super::cons::CONS;
use super::super::super::cons::ErrCode;

use std::collections::BTreeMap;
use std::io::Read;
use std::str::FromStr;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

extern crate regex;
use self::regex::Regex;

extern crate time;

use super::super::inter::{DataApi};
use super::super::util::{KeyHelper};

//upload file block 
pub struct F03;

impl DataApi for F03 {

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
        let body = msg.find_path(&["body"]).unwrap();
        let user_id_str = msg.find_path(&["head", "userId"]).unwrap().as_string().unwrap();
        let customer_id = i64::from_str(user_id_str).unwrap();
        let file_id = body.find_path(&["file_id"]).unwrap().as_i64().unwrap();
        let index = body.find_path(&["index"]).unwrap().as_i64().unwrap();
        let file_block_id = format!("{}_{}_{}", customer_id, file_id, index);
        let table = db.get_table("file_block").expect("file_block table not exits.");
        let mut file_block = body.clone();
        {
            let mut body_obj = file_block.as_object_mut().unwrap();
            body_obj.insert("id".to_string(), file_block_id.to_json());
            body_obj.insert("customer_id".to_string(), customer_id.to_json());
        }
        let op = Json::from_str("{}").unwrap();
        let rst = table.save(&file_block, &op);
        Result::Ok(Json::from_str("{}").unwrap())
    }

}
