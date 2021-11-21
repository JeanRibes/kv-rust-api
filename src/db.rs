use std::sync::Arc;

use crate::models::Kudos;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tokio::time::{Duration, sleep};
use tokio::signal;
use std::process::exit;
use tokio::sync::Mutex;
use serde_json::from_reader;

pub type Db = Arc<Mutex<HashMap<String, Kudos>>>;

pub fn init_db() -> Db {
    let file = File::open(get_filename());
    let data = match file {
        Ok(json) => {
            from_reader(json).expect("cannot read db.json")
        }
        Err(_) => {
            HashMap::new()
        }
    };
    return Arc::new(Mutex::new(data));
}

pub fn save_db(db: HashMap<String, Kudos>) {
    let file = File::create(get_filename());
    match file {
        Ok(mut json) => {
            //db.serialize(serde_json::ser::Serializer::new(json));
            json.write(serde_json::to_string(&db).expect("lol").as_ref()).expect("could not write to file");
            //to_writer(json, &db);
        }
        Err(_) => {
            match serde_json::to_string_pretty(&db) {
                Ok(value) => {
                    println!("could not write DB to file 'db.json', here is your data: {}", value);
                }
                Err(e) => {
                    println!("serializeation error !!: {}", e);
                }
            };
        }
    };
}

fn get_filename() -> String {
    match std::env::var("DB_FILE") {
        Ok(filename) => { filename }
        Err(_) => { "db.json".to_string() }
    }
}

async fn hash_db(db: &Db) -> i64 {
    db.lock().await.iter().fold(0,|acc, kv| { //Functionnal Programming yay!
        let (_,count) = kv;
        return acc + count;
    })
}

pub async fn sync_db(db: &Db) {
    println!("Syncing database");
    save_db(db.lock().await.clone());
}


pub async fn save_daemon(db: Db) {
    let save_interval = Duration::new(300, 0);
    let mut last_hash = hash_db(&db).await;
    loop {
        let hash = hash_db(&db).await;
        if hash != last_hash {
            sync_db(&db).await;
            last_hash = hash;
        }
        sleep(save_interval).await;
    }
}

pub async fn save_exit(db: Db) {
    signal::ctrl_c().await.expect("couldn't listen to signal");
    println!("SIGTERM...");
    sync_db(&db).await;
    println!("DB Saved, exiting !");
    exit(0);
}