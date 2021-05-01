use env_logger;
use log::debug;
use std::env;

mod config;
mod farmbot;
mod i2cproxy;
mod mqtt;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let config_file = args.get(1).expect("usage: farmbot config.toml");

    debug!("Reading config file");
    let config = config::get(config_file.to_string()).expect("Error loading config file");

    let mut farmbot = farmbot::Farmbot::new(&config).expect("creating farmbot");
    farmbot.start_mqtt();
    farmbot.init_sensors();
    farmbot.start_btle();
    farmbot.run();
}
