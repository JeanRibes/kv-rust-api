use tokio::task;

mod models;
mod db;
mod handlers;
mod routes;


#[tokio::main]
async fn main() {
    println!("hello, look at 0.0.0.0:3030");
    let db = db::init_db();
    let routes = routes::routes(db.clone());

    task::spawn(db::save_daemon(db.clone()));

    task::spawn(db::save_exit(db.clone()));

    warp::serve(routes).run(([0,0,0,0], 3030)).await;
    println!("end")
}