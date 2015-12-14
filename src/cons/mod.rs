use std::collections::BTreeMap;
use std::sync::{Arc};


pub enum ErrCode {
    Success = 0,

    DigestFailure,
    TokenExpired,
    ValueNotExist,

    UsernameIsNull,
    UsernameWrongPattern,
    UsernameIsWrong,

    PasswordIsNull,
    PasswordWrongPattern,

    UserInfoIsWrong,

}

pub struct ConsNode {
    id:i32,
    code:String,
    desc:String,
}

pub struct Cons {
    code_data:BTreeMap<String, Arc<ConsNode>>,
    id_data:BTreeMap<i32, Arc<ConsNode>>,
}



impl Cons {

    pub fn from_vec(vec:Vec<Arc<ConsNode>>) -> Cons {
        let mut code_data:BTreeMap<String, Arc<ConsNode>> = BTreeMap::new();
        let mut id_data:BTreeMap<i32, Arc<ConsNode>> = BTreeMap::new();
        for node in vec {
            code_data.insert(node.code.clone(), node.clone());
            id_data.insert(node.id, node);
        }
        Cons {
            code_data: code_data,
            id_data: id_data,
        }
    }

}

pub struct ConsFactory {
    cons:BTreeMap<String, Cons>,
}

impl ConsFactory {

    pub fn new() -> ConsFactory {
        let mut cons:BTreeMap<String, Cons> = BTreeMap::new();
        let user_type_vec = vec![
            Arc::new(ConsNode{id:100, code:"guest".to_string(), desc:"游客".to_string()}),
            Arc::new(ConsNode{id:200, code:"normal".to_string(), desc:"普通用户".to_string()}),
            Arc::new(ConsNode{id:900, code:"admin".to_string(), desc:"管理员".to_string()}),
        ];
        cons.insert("user_type".to_string(), Cons::from_vec(user_type_vec));
        let file_type_vec = vec![
            Arc::new(ConsNode{id:-1, code:"unknown".to_string(), desc:"未知".to_string()}),
            Arc::new(ConsNode{id:100, code:"text/xml".to_string(), desc:"xml".to_string()}),
            Arc::new(ConsNode{id:200, code:"text/plain".to_string(), desc:"txt".to_string()}),
        ];
        cons.insert("file_type".to_string(), Cons::from_vec(file_type_vec));

        ConsFactory{
            cons:cons,
        }
    }

    pub fn by_id(&self, name:&str, id:i32) -> Option<&Arc<ConsNode>> {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.id_data.get(&id)
    }

    pub fn code_to_id(&self, name:&str, code:&str) -> Result<i32, i32> {
        //println!("the name is {}.", name);
        //println!("the code is {}.", code);
        let op = self.by_code(name, code);
        match op {
            Some(x) => {
                Result::Ok((**x).id)
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }

    pub fn by_code(&self, name:&str, code:&str) -> Option<&Arc<ConsNode>> {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.code_data.get(code)
    }

    pub fn id_to_code(&self, name:&str, id:i32) -> Result<String, i32> {
        let op = self.by_id(name, id);
        match op {
            Some(x) => {
                Result::Ok((**x).code.clone())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }

}

lazy_static! {
    pub static ref CONS:ConsFactory = ConsFactory::new();
}





