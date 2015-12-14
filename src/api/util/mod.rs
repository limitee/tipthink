use std::str::FromStr;

use super::super::util::DigestUtil;
use super::super::dc::DataBase;
use super::super::dc::MyDbPool;
use super::super::cons::CONS;
use super::super::cons::ErrCode;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

pub struct KeyHelper;

extern crate time;

impl KeyHelper {

    /**
     * get key from cache.
     */
    pub fn from_cache(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let user_id_str = head.find_path(&["userId"]).unwrap().as_string().unwrap();
        let user_id = i64::from_str(user_id_str).unwrap(); 
        let st_table = db.get_table("st").expect("st table not exists.");
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, user_id);
        let fd_back = st_table.find_by_str(&cond, "{}", "{}");
        let rows = fd_back.find_path(&["rows"]).unwrap().as_i64().unwrap();
        if(rows > 0) 
        {
            let mut data = fd_back.find_path(&["data"]).unwrap().as_array().unwrap();
            let last_active_time = data[0].find_path(&["last_active_time"]).unwrap().as_i64().unwrap();
            let sec = time::get_time().sec;
            if(sec - last_active_time > 120000) 
            {
                Result::Err(ErrCode::TokenExpired as i32)
            }
            else
            {
                let st = data[0].find_path(&["st"]).unwrap().as_string().unwrap();
                Result::Ok(st.to_string())
            }
        }
        else
        {
            Result::Err(ErrCode::TokenExpired as i32)            
        }
    }

    pub fn active(db:&DataBase<MyDbPool>, head:&Json) {
        let user_id_str = head.find_path(&["userId"]).unwrap().as_string().unwrap();
        let user_id = i64::from_str(user_id_str).unwrap(); 
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, user_id);
        let sec = time::get_time().sec;
        let doc = format!(r#"
        {{
            "$set":
                {{
                    "last_active_time":{}
                }}
            }}
        "#, sec);
        let st_table = db.get_table("st").expect("st table not exists.");
        st_table.update_by_str(&cond, &doc, "{}");
    }

    pub fn from_db(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let username = head.find_path(&["userId"]).unwrap().as_string().unwrap();
        let user_type = head.find_path(&["userType"]).unwrap().as_string().unwrap();

        let rst = CONS.code_to_id("user_type", &user_type);
        rst.and_then(|user_type_id| {
            let table = db.get_table("customer").expect("table not exists.");
            let cond = format!(r#"
               {{
                   "username":"{}",
                   "type":{}
               }}
            "#, username, user_type_id);
            let fd_back = table.find_by_str(&cond, "{}", "{}");
            let fd_back_obj = fd_back.as_object().unwrap();
            let rows = fd_back_obj.get("rows").unwrap().as_i64().unwrap();
            if rows > 0 {   //return the db object to client
                let mut data_array = fd_back_obj.get("data").unwrap().as_array().unwrap();
                let user = data_array.get(0).unwrap();
                let password = user.as_object().unwrap().get("password").unwrap().as_string().unwrap();
                let digest = DigestUtil::md5(password);
                return Result::Ok(digest)
            }
            else
            {
               return Result::Err(ErrCode::UserInfoIsWrong as i32);
            }
        })
    }

}

