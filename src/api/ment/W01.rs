use super::super::super::util::DigestUtil;
use super::super::super::dc::DataBase;
use super::super::super::dc::MyDbPool;

use std::collections::BTreeMap;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

extern crate regex;
use self::regex::Regex;

extern crate time;

use super::super::inter::{DataApi};

extern crate hyper;
use self::hyper::client::Client;

pub struct W01;

impl DataApi for W01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        Result::Ok("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string())
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let table = db.get_table("weather").expect("table not exists.");

        let now = time::now();
        let year = now.tm_year + 1900;
        let month = now.tm_mon + 1;
        let day = now.tm_mday;
        let date = format!("{:04}-{:02}-{:02}", year, month, day);

        let cond = "{\"date\":\"".to_string() + &date + "\"}";
        let mut fd_back = table.find_by_str(&cond, "{}", "{}");
        let mut fd_back_obj = fd_back.as_object_mut().unwrap();
        let rows = fd_back_obj.get("rows").unwrap().as_i64().unwrap();
        let back = {
            if rows > 0 {   //return the db object to client
                let mut data_array = fd_back_obj.get_mut("data").unwrap().as_array_mut().unwrap();
                let weather = data_array.remove(0);
                weather
            }
            else
            {
                let client = Client::new();
                let mut res = client.get("http://apistore.baidu.com/microservice/weather?cityid=101160801").send().unwrap();
                let mut content = String::new();
                res.read_to_string(&mut content);

                let wt_json = Json::from_str(&content).unwrap();
                let ret_data_json = wt_json.as_object().unwrap().get("retData").unwrap();

                let wind_direc = ret_data_json.find("WD").unwrap().as_string().unwrap();
                let wind_detail = ret_data_json.find("WS").unwrap().as_string().unwrap();
                let wind = wind_direc.to_string() + " " + wind_detail;
                let detail = ret_data_json.find("weather").unwrap().as_string().unwrap();
                let h_tmp = ret_data_json.find("h_tmp").unwrap().as_string().unwrap();
                let l_tmp = ret_data_json.find("l_tmp").unwrap().as_string().unwrap();
                let temp = format!("{} ~ {}", l_tmp, h_tmp);

                let mut map = BTreeMap::new();
                map.insert("wind".to_string(), wind.to_json());
                map.insert("temp".to_string(), temp.to_json());
                map.insert("detail".to_string(), detail.to_json());
                map.insert("date".to_string(), date.to_json());
                let back_wt_json = map.to_json();

                table.save(&back_wt_json, &(Json::from_str("{}").unwrap()));
                back_wt_json
            }
        };
        Result::Ok(back)
    }

}
