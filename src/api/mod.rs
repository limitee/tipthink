use std::collections::BTreeMap;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

use super::util::DigestUtil;
use super::dc::DataBase;
use super::dc::MyDbPool;

use super::cons;
use super::cons::ErrCode;

extern crate time;

extern crate hyper;
use self::hyper::client::Client;

extern crate regex;
use self::regex::Regex;

mod inter;
use self::inter::{DataApi};

mod util;
use self::util::{KeyHelper};

mod ment;
use self::ment::U01::U01;
use self::ment::U02::U02;
use self::ment::U03::U03;
use self::ment::W01::W01;
use self::ment::F01::F01;
use self::ment::F02::F02;
use self::ment::F03::F03;
use self::ment::SV01::SV01;
use self::ment::BKT01::BKT01;
use self::ment::BKT02::BKT02;
use self::ment::BKT03::BKT03;

pub struct ApiFactory {
    map:BTreeMap<String, Box<DataApi>>,
}

impl ApiFactory {

    pub fn new() -> ApiFactory {
        let mut map = BTreeMap::new();
        map.insert("U01".to_string(), Box::new(U01) as Box<DataApi>);
        map.insert("U02".to_string(), Box::new(U02) as Box<DataApi>);
        map.insert("U03".to_string(), Box::new(U03) as Box<DataApi>);
        map.insert("W01".to_string(), Box::new(W01) as Box<DataApi>);
        map.insert("F01".to_string(), Box::new(F01) as Box<DataApi>);
        map.insert("F02".to_string(), Box::new(F02) as Box<DataApi>);
        map.insert("F03".to_string(), Box::new(F03) as Box<DataApi>);
        map.insert("SV01".to_string(), Box::new(SV01) as Box<DataApi>);
        map.insert("BKT01".to_string(), Box::new(BKT01) as Box<DataApi>);
        map.insert("BKT02".to_string(), Box::new(BKT02) as Box<DataApi>);
        map.insert("BKT03".to_string(), Box::new(BKT03) as Box<DataApi>);
        ApiFactory {
            map:map,
        }
    }

    /**
     * get the digest key by head.
     */
    pub fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let name = {
            head.as_object().unwrap().get("cmd").unwrap().as_string().unwrap()
        };
        let api = self.map.get(name).unwrap();
        api.get_key(db, head)
    }

    /**
     * check the digest. If success return Some, else return None.
     */
    pub fn check(&self, db:&DataBase<MyDbPool>, param:&BTreeMap<String, String>) -> Result<Json, i32> {
        let head = param.get("head").unwrap();
        let head_node = Json::from_str(head).unwrap();
        let head_node_obj = head_node.as_object().unwrap();
        let digest = head_node_obj.get("digest").unwrap().as_string().unwrap();
        let time_stamp = head_node_obj.get("timeStamp").unwrap().as_string().unwrap();

        let key_rst = self.get_key(db, &head_node);
        key_rst.and_then(|key| {
            let body_str = param.get("body").unwrap();
            let digest_content = format!("{}{}{}", key, body_str, time_stamp);
            if digest == DigestUtil::md5(&digest_content) {
                let body_node = Json::from_str(body_str).unwrap();
                let mut back_obj = BTreeMap::new();
                back_obj.insert("head".to_string(), head_node_obj.to_json());
                back_obj.insert("body".to_string(), body_node);
                back_obj.insert("key".to_string(), key.to_json());
                Result::Ok(back_obj.to_json())
            }
            else
            {
                Result::Err(ErrCode::DigestFailure as i32)
            }
        })
    }

    pub fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let name = {
            msg.as_object().unwrap().get("head").unwrap().as_object().unwrap().get("cmd").unwrap().as_string().unwrap()
        };
        let api = self.map.get(name).unwrap();
        api.check(db, msg).and_then(|flag|{
            api.run(db, msg) 
        })
    }


    pub fn back(&self, msg:&Json, body:String) -> Json {
        let msg_obj = msg.as_object().unwrap();
        let head = msg_obj.get("head").unwrap();
        let time = head.as_object().unwrap().get("timeStamp").unwrap().as_string().unwrap();
        let key = msg_obj.get("key").unwrap().as_string().unwrap();

        let digest_content = format!("{}{}{}", key, body, time);
        println!("{}", digest_content);
        let digest = DigestUtil::md5(&digest_content);
        println!("{}", digest);

        let mut back_head = head.clone();
        {
            let mut back_head_obj = back_head.as_object_mut().unwrap();
            back_head_obj.insert("digest".to_string(), digest.to_json());
        }

        let mut back_obj = BTreeMap::new();
        back_obj.insert("head".to_string(), back_head);
        back_obj.insert("body".to_string(), body.to_json());
        back_obj.to_json()
    }

    pub fn back_err(&self, head:&Json, body:String) -> Json {
        let mut back_head = head.clone();
        {
            let head_obj = head.as_object().unwrap();
            let time = head_obj.get("timeStamp").unwrap().as_string().unwrap();
            let key = DigestUtil::empty_key();
            let digest_content = format!("{}{}{}", key, body, time);
            let digest = DigestUtil::md5(&digest_content);

            let mut back_head_obj = back_head.as_object_mut().unwrap();
            back_head_obj.insert("digestType".to_string(), "md5-empty".to_json());
            back_head_obj.insert("digest".to_string(), digest.to_json());
        }
        let mut back_obj = BTreeMap::new();
        back_obj.insert("head".to_string(), back_head);
        back_obj.insert("body".to_string(), body.to_json());
        back_obj.to_json()
    }
}
