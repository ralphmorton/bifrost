
use axum::extract::Extension;
use axum::routing;
use axum::Router;
use axum::Server;
use clap::Parser;
use heimdall::handlers;
use heimdall::registry::Registry;
use heimdall::resolver::disk::DiskResolver;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let resolver = DiskResolver::new(args.module_dir);
    let registry = Registry::new(Box::new(resolver), args.max_cached_modules);

    let app =
        Router::new()
        .route("/:module_id", routing::post(handlers::recv))
        .layer(Extension(Arc::new(registry)));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(args.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        args.port
    ));

    Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Address to listen on
    #[arg(long = "addr", default_value = "::1")]
    addr: String,

    /// Port to listen on
    #[arg(long = "port")]
    pub port: u16,

    /// Maximum number of modules to allow in cache
    #[arg(long = "cache")]
    pub max_cached_modules: u64,

    /// Module directory, specify to use disk storage mechanism for modules
    #[arg(long = "dir")]
    pub module_dir: String
}
