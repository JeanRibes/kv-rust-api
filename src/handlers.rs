use std::convert::Infallible;
use warp;

use crate::db::Db;
use crate::db::save_db;
use warp::http::StatusCode;
use crate::models::Kudos;
use std::borrow::{BorrowMut, Borrow};
use std::collections::HashMap;
use std::sync::MutexGuard;
use serde::de::IntoDeserializer;

pub async fn list_kudos(db: Db) -> Result<impl warp::Reply, Infallible> {
    let store = db.lock().await;
    Ok(warp::reply::json(&store.clone()))
}

pub async fn get_kudos(slug: String,db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let store = db.lock().await;
    let obj = store.get(slug.as_str());
    //Ok(warp::reply::json(&obj.clone()))
    match obj {
        None => {
            return Ok(Box::new(StatusCode::NOT_FOUND))
        }
        Some(kudos) => {
            return Ok(Box::new(warp::reply::json(&kudos.count.clone())))
        }
    }
}

pub async fn send_kudos(slug: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    let slug_insert = slug.clone();
    let slug_get = slug.clone();
    match store.get_mut(&slug_get) {
        Some(kudos) => {
            kudos.count+=1;
            return Ok(warp::reply::json(&kudos.count))
            /*let mut kudos = kudos.clone();
            kudos.count +=1;
            return Ok(warp::reply::json(&kudos));*/
        }
        None => {
            let kudos = Kudos{
                count: 1,
                slug
            };
            store.insert(slug_insert,kudos.clone());
            return Ok(warp::reply::json(&kudos));
        }
    }
}

pub async fn flush_db(db: Db) -> Result<impl warp::Reply, Infallible> {
    println!("flushing DB");
    let store = db.lock().await;
    save_db(store.clone());
    Ok(StatusCode::OK)
}