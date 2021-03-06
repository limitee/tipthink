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

//move the index of book type 
pub struct BKT03;

impl DataApi for BKT03 {

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
        let table = db.get_table("book_type").expect("st table not exists.");
        let book_type_id = json_i64!(msg; "body", "id");
        let up_or_down = json_i64!(msg; "body", "up_or_down");
        let index = json_i64!(msg; "body", "index");
        let t_index = index + up_or_down;
        let rst = {
            let cond = format!(r#"
                {{
                    "index":{}
                }}
            "#, t_index);
            let doc = format!(r#"
                {{
                    "$set":
                    {{
                        "index":{}
                    }}
                }}
            "#, index);
            let op = r#"
                {
                   "ret": 
                   {
                       "id": 1
                   }
                }
            "#;
            let data = table.update_by_str(&cond, &doc, &op);
            println!("{}", data);
            let row = json_i64!(&data; "rows");
            row
        };
        if rst > 0 {
            let cond = format!(r#"
                {{
                    "id":{}
                }}
            "#, book_type_id);
            let doc = format!(r#"
                {{
                    "$set":
                    {{
                        "index":{}
                    }}
                }}
            "#, t_index);
            table.update_by_str(&cond, &doc, "{}");
        }
        Result::Ok(json!("{}"))
    }

}
