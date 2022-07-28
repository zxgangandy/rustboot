mod utils;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use log::{info, error};
use log4rs;
use rbatis::rbatis::Rbatis;
use rbatis::db::DBPoolOptions;
use rocket::fairing::AdHoc;
use utils::errors;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[rocket::main]
async fn main() -> errors::Result<()> {
    println!("Hello, world!");

    log4rs::init_file("config/config.yaml", Default::default()).unwrap();
    info!("hnt tools booting up");
    error!("Connecting to database...");

    let mut opt = DBPoolOptions::new();
    opt.max_connections = 50;
    RB.link_opt("mysql://root:root@127.0.0.1:3306/test", opt)
        .await
        .expect("Rbatis connect database failed!!!");
    let rb = Arc::new(&RB);

    let _ = rocket::build()
        //.register("/", catchers![not_found])
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
            ],
        )
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await?;

    Ok(())
}
