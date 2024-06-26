mod arg;
mod config;
mod server;
mod shutdown;
use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    xlog::log::init(std::io::stdout(), xlog::log::Level::Trace);
    let args = arg::Args::parse();
    match config::init() {
        Ok(amc) => {
            let signal = shutdown::init();
            tonic::transport::Server::builder()
                .add_service(server::MainServer::new(server::Main::new(amc.clone())))
                .add_service(server::ExtServer::new(server::Ext::new(amc.clone())))
                .serve_with_shutdown(
                    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.port),
                    signal,
                )
                .await?;
            if let Err(e) = amc.save() {
                xlog::error!("fail to save config: {:?}", e);
            }
        }
        Err(e) => {
            xlog::error!("fail to init config: {:?}", e);
        }
    }
    Ok(())
}
