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

//upload file 
pub struct F02;

impl DataApi for F02 {

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
        let user_id_str = msg.find_path(&["head", "userId"]).unwrap().as_string().unwrap();
        let customer_id = i64::from_str(user_id_str).unwrap();

        let req_body = msg.find_path(&["body"]).unwrap(); 
        let file_type = req_body.find_path(&["type"]).unwrap().as_string().unwrap();
        let file_type_id = match CONS.code_to_id("file_type", file_type) {
            Ok(x) => x,
            Err(_) => -1,
        };
        let mut body = req_body.clone();
        {
            let mut body_obj = body.as_object_mut().unwrap();
            body_obj.insert("type".to_string(), file_type_id.to_json());
            let sec = time::get_time().sec;
            body_obj.insert("create_time".to_string(), sec.to_json());
            body_obj.insert("customer_id".to_string(), customer_id.to_json());
        }
        println!("{}", body);

        let table = db.get_table("file").expect("file table not exists.");
        let op = r#"
            {
                "ret":
                {
                    "id":1
                }
            }
        "#;
        let op_json = Json::from_str(op).unwrap();
        let data = table.save(&body, &op_json);
        let data_array = data.find_path(&["data"]).unwrap().as_array().unwrap();
        let id = data_array[0].find_path(&["id"]).unwrap().as_i64().unwrap();
        let back_body = format!(r#"
            {{
                "id":{}
            }}
        "#, id);
        Result::Ok(Json::from_str(&back_body).unwrap())
    }

}
