use warp::Filter;

mod models;
mod db;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let db = db::init_db();
    let routes = routes::routes(db);


    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}