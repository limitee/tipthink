use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate easydb;
use self::easydb::Column;
use self::easydb::Table;
use self::easydb::DbPool;

use std::collections::BTreeMap;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

extern crate postgres;
use self::postgres::{Connection, SslMode};
use self::postgres::types::Type;

extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

pub struct MyDbPool {
    dsn:String,
    conns:Vec<Mutex<Connection>>,
}

impl MyDbPool {

    pub fn new(dsn:&str, size:u32) -> MyDbPool {
        let mut conns = vec![];
        for i in 0..size {
            let conn = match Connection::connect(dsn, &SslMode::None) {
                Ok(conn) => conn,
                Err(e) => {
                    println!("Connection error: {}", e);
                    break;
                }
            };
            conns.push(Mutex::new(conn));
        }
        MyDbPool {
            dsn:dsn.to_string(),
            conns:conns,
        }
    }

}

impl DbPool for MyDbPool {

    fn execute(&self, sql:&str) -> Json {
        println!("{}", sql);
        let between = Range::new(0, self.conns.len());
        let mut rng = rand::thread_rng();
        let rand_int = between.ind_sample(&mut rng);
        let conn = self.conns[rand_int].lock().unwrap();
        let stmt = conn.prepare(&sql).unwrap();
        let rows = stmt.query(&[]).unwrap();
        let mut back_obj = BTreeMap::new();
        let mut data:Vec<Json> = Vec::new();
        for row in &rows {
            let mut row_map = BTreeMap::new();
            let columns = row.columns();
            for column in columns {
                let name = column.name();
                match *column.type_() {
                    Type::Int4 => {
                        let value:i32 = row.get(name);
                        row_map.insert(name.to_string(), value.to_json());
                    },
                    Type::Int8 => {
                        let value:i64 = row.get(name);
                        row_map.insert(name.to_string(), value.to_json());
                    },
                    Type::Timestamp => {

                    },
                    _ => {
                        let value:String = row.get(name);
                        row_map.insert(name.to_string(), value.to_json());
                    },
                }
            }
            data.push(row_map.to_json());
        }
        back_obj.insert("data".to_string(), data.to_json());
        back_obj.insert("rows".to_string(), rows.len().to_json());
        back_obj.to_json()
    }

}

pub struct DataBase<T> {
    pub name:String,
    pub table_list:BTreeMap<String, Table<T>>,
    pub dc:Arc<T>,   //data center
}

impl<T:DbPool> DataBase<T> {

    fn get_table_define(name:&str, vec:Vec<Column>, dc:Arc<T>) -> Table<T>
    {
        let mut map = BTreeMap::new();
        for col in vec {
            map.insert(col.name.clone(), col);
        }
        Table::new(name, map, dc)
    }

    pub fn new(name:&str, dc:Arc<T>) -> DataBase<T>
    {
        let mut table_list = BTreeMap::new();
        {
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "serial", -1, "", false),
                Column::new("date", "varchar", 20, "not null", false),
                Column::new("temp", "varchar", 80, "not null", true),
                Column::new("detail", "varchar", 80, "not null", true),
                Column::new("wind", "varchar", 80, "not null", true),
                Column::new("create_time", "timestamp", -1, "default now()", false),
            ];
            let table = DataBase::get_table_define("weather", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the user's st
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigint", -1, "unique not null", false),
                Column::new("st", "varchar", 32, "not null default ''", false),
                Column::new("fix_st", "varchar", 32, "not null default ''", false),
                Column::new("role", "integer", -1, "default -1", false),
                Column::new("last_active_time", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("st", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the customer
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("username", "varchar", 40, "unique not null", false),
                Column::new("nickname", "varchar", 40, "not null default ''", true),
                Column::new("password", "varchar", 40, "not null", false),
                Column::new("reg_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("avatar_id", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("customer", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the file table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "", false),
                Column::new("name", "varchar", 80, "not null", false),
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("type", "integer", -1, "default -1", false),
                Column::new("size", "bigint", -1, "default -1", false),
                Column::new("customer_id", "bigint", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("file", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the file block table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "varchar", 80, "PRIMARY KEY", false),
                Column::new("file_id", "bigint", -1, "", false),
                Column::new("customer_id", "bigint", -1, "", false),
                Column::new("start", "bigint", -1, "", false),
                Column::new("index", "int", -1, "", false),
                Column::new("size", "int", -1, "", false),
                Column::new("content", "text", -1, "not null", false),
            ];
            let table = DataBase::get_table_define("file_block", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the save info table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("title", "varchar", 200, "", true),
                Column::new("customer_id", "bigint", -1, "", false),
                Column::new("content", "text", -1, "not null", true),
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("status", "int", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("save_info", vec, dc);
            table_list.insert(table.name.clone(), table);
        }
        {   //the book_type table
            let dc = dc.clone();
            let vec = vec![
                Column::new("id", "bigserial", -1, "PRIMARY KEY", false),
                Column::new("name", "varchar", 40, "", true),
                Column::new("create_time", "bigint", -1, "default -1", false),
                Column::new("index", "int", -1, "default -1", false),
                Column::new("version", "int", -1, "default -1", false),
            ];
            let table = DataBase::get_table_define("book_type", vec, dc);
            table_list.insert(table.name.clone(), table);
        }

        for (name, table) in table_list.iter() {
            println!("{}", table.to_ddl_string());
        }
        DataBase {
            name:name.to_string(),
            table_list:table_list,
            dc:dc,
        }
    }

    pub fn get_table(&self, name:&str) -> Option<&Table<T>>
    {
        self.table_list.get(name)
    }

}
