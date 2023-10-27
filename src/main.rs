mod config;
mod server;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
};
use xlog_rs::log;

pub struct Flags {
    pub port: u16,
}
fn show_help() {
    println!("Usage: qst [options]");
    println!("Options:");
    println!("  --port <port>    set port");
    println!("  --help         show help");
}
impl Flags {
    pub fn new(args: Vec<String>) -> Self {
        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "--help" => {
                    show_help();
                    std::process::exit(0);
                }
                "--port" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse::<u16>() {
                            Err(e) => {
                                log::error(format!("invalid port: {}", e).as_str());
                                show_help();
                                std::process::exit(1);
                            }
                            Ok(p) => {
                                log::info(format!("port: {}", p).as_str());
                                return Self { port: p };
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        println!("invalid args");
        show_help();
        std::process::exit(1);
    }
}

#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
    let flag = Flags::new(std::env::args().collect());

    let amc = Arc::new(Mutex::new(config::Config::new()));
    tonic::transport::Server::builder()
        .add_service(server::MainServer::new(server::Main::new(&amc)))
        .add_service(server::ExtServer::new(server::Ext::new(&amc)))
        .serve(SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            flag.port,
        ))
        .await
    // let mut cfg = config::Config::new();
    // cfg.file.exts.insert(
    //     0,
    //     config::Appattr {
    //         name: "qst-ext-appseracher-rust".to_string(),
    //         prompt: "".to_string(),
    //         dir: "/home/kmdr/pro/qst-ext-appsearcher-rust".to_string(),
    //         exec: "ls".to_string(),
    //         addr: "".to_string(),
    //     },
    // );
    // cfg.save();
    // println!("{:?}", cfg);
}
