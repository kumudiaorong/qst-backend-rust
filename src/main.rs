mod arg;
mod config;
mod server;
use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
    let args = arg::Args::parse();
    let amc = config::init();
    tonic::transport::Server::builder()
        .add_service(server::MainServer::new(server::Main::new(amc.clone())))
        .add_service(server::ExtServer::new(server::Ext::new(amc)))
        .serve(SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            args.port,
        ))
        .await
}
