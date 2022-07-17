#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

use log::{info, error};
use log4rs;

// 定义全局变量
lazy_static! {
    // Rbatis类型变量 RB，用于数据库查询
    static ref RB: Rbatis = Rbatis::new();
}

#[rocket::main]
async fn main() {
    println!("Hello, world!");

    log4rs::init_file("config/config.yaml", Default::default()).unwrap();
    info!("hnt tools booting up");
    error!("Connecting to database...");
}
