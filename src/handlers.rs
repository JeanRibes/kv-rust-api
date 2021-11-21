use std::convert::Infallible;
use warp;

use crate::db::Db;
use crate::db::save_db;
use warp::http::StatusCode;

pub async fn list_kudos(db: Db) -> Result<impl warp::Reply, Infallible> {
    let store = db.lock().await;
    Ok(warp::reply::json(&store.clone()))
}

pub async fn get_kudos(slug: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let store = db.lock().await;
    let kudos = match store.get(&slug) {
        Some(kudos) => kudos,
        None => &0,
    };
    Ok(warp::reply::json(kudos))
}

pub async fn send_kudos(slug: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;

    let kudos = match store.get(&slug) {
        Some(kudos) => *kudos + 1,
        None => 1
    };
    store.insert(slug, kudos);
    Ok(warp::reply::json(&kudos))
}

pub async fn flush_db(db: Db) -> Result<impl warp::Reply, Infallible> {
    println!("flushing DB");
    let store = db.lock().await;
    save_db(store.clone());
    Ok(StatusCode::OK)
}
