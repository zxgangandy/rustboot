
use log::{info, error};
use log4rs;


fn main() {
    println!("Hello, world!");

    log4rs::init_file("config/config.yaml", Default::default()).unwrap();
    info!("hnt tools booting up");
    error!("Connecting to database...");
}
