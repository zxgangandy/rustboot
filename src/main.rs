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
use rocket::fairing::AdHoc;
use utils::errors;

// 定义全局变量
lazy_static! {
    // Rbatis类型变量 RB，用于数据库查询
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

    RB.link("mysql://root:root@127.0.0.1:3306/test")
        .await
        .unwrap();
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
