
use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Kudos;
use warp::filters::BoxedFilter;
use warp::filters::path::Exact;
use tokio::macros::support::Future;

pub fn routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone  {
    get_kudos(db.clone())
        .or(send_kudos(db.clone()))
        .or(flush_db(db.clone()))
        .or(list_kudos(db)) // pas besoin de clone pour la dernière route
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn send_kudos(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    return warp::path!("kudos" / String)
        .and(warp::post())
        .and(with_db(db))
        .and_then(handlers::send_kudos);
}

fn list_kudos(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
     warp::path("all")
         .and(warp::get())
         .and(with_db(db))
         .and_then(handlers::list_kudos)
}

fn get_kudos(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("kudos" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_kudos)
}

fn flush_db(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("save")
        .and(warp::any())
        .and(with_db(db))
        .and_then(handlers::flush_db)
}
