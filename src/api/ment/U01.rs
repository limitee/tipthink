use super::super::super::dc::DataBase;
use super::super::super::dc::MyDbPool;
use super::super::super::cons::ErrCode;
use super::super::super::cons::CONS;

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

//user register
pub struct U01;

impl DataApi for U01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        Result::Ok("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string())
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        let body = msg.as_object().unwrap().get("body").unwrap();
        let body_obj = body.as_object().unwrap();
        let username_node = body_obj.get("username");
        match username_node {
            Some(x) => {
                if x.as_string().is_none() {
                    return Result::Err(ErrCode::UsernameWrongPattern as i32);
                }
                else
                {
                    let username = x.as_string().unwrap();
                    println!("{}", username);
                    let re = Regex::new(r"^[a-z|A-Z]{1}[a-z|A-Z|0-9]{5, 19}$").unwrap();
                    if !re.is_match(username) {
                        return Result::Err(ErrCode::UsernameWrongPattern as i32);
                    }
                }
            },
            None => {
                return Result::Err(ErrCode::UsernameIsNull as i32);
            },
        }
        let password_node = body_obj.get("password");
        match password_node {
            Some(x) => {
                if x.as_string().is_none() {
                    return Result::Err(ErrCode::PasswordWrongPattern as i32);
                }
                else
                {
                    let password = x.as_string().unwrap();
                    let re = Regex::new(r"^[a-z|A-Z|0-9|#|@|!]{6, 20}$").unwrap();
                    if !re.is_match(password) {
                        return Result::Err(ErrCode::PasswordWrongPattern as i32);
                    }
                }
            },
            None => {
                return Result::Err(ErrCode::PasswordIsNull as i32);
            }
        }
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let table = db.get_table("customer").expect("table not exists.");
        let body = msg.as_object().unwrap().get("body").unwrap();

        let username = json_str!(body; "username");
        let password = json_str!(body; "password");
        let now = time::get_time();
        let user_type = CONS.code_to_id("user_type", "normal").unwrap();
        let save_obj_str = format!(r#"
            {{
                "username":"{}",
                "password":"{}",
                "reg_time": {},
                "type": {}
            }}
        "#, username, password, now.sec, user_type);
        let save_obj = json!(&save_obj_str); 
        let op = json!("{}");
        table.save(&save_obj, &op);
        Result::Ok(Json::from_str("{}").unwrap())
    }

}
