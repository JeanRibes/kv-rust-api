use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

use crate::models::Kudos;
use std::collections::HashMap;
use std::fs::File;
use serde_json::{from_reader, Error};
use serde_json::to_writer;
use serde_json::to_string;
use serde_json::value::Serializer;
use serde::Serialize;
use std::io::Write;


pub type Db = Arc<Mutex<HashMap<String,Kudos>>>;

pub fn init_db() -> Db {
    let file = File::open("db.json");
    match file {
        Ok(json) => {
            let data = from_reader(json).expect("cannot read db.json");
            return Arc::new(Mutex::new(data));
        }
        Err(_) => {
            return Arc::new(Mutex::new(HashMap::new()));
        }
    }
}

pub fn save_db(db: HashMap<String,Kudos>){
    let file = File::create("db.json");
    match file {
        Ok(mut json) => {
            //db.serialize(serde_json::ser::Serializer::new(json));
            json.write(serde_json::to_string(&db).expect("lol").as_ref());
            //to_writer(json, &db);
        }
        Err(_) => {
            match serde_json::to_string_pretty(&db) {
                Ok(value) => {
                    println!("could not write DB to file 'db.json', here is your data: {}", value);
                }
                Err(e) => {
                    println!("serializeation error !!: {}",e);
                }
            };
        }
    };
}