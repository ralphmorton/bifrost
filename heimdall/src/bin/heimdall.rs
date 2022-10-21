use axum::extract::Extension;
use axum::routing;
use axum::Router;
use axum::Server;
use clap::Parser;
use heimdall::handlers;
use heimdall::registry::Registry;
use heimdall::store::disk::DiskStore;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{},hyper=info,mio=info,cranelift_codegen=error,wasmtime=error,wasmtime_cranelift=error", args.log_level)
        )
    }

    tracing_subscriber::fmt::init();

    let store = DiskStore::new(args.module_dir);
    let registry = Registry::new(Box::new(store), args.max_cached_modules);

    let app = Router::new()
        .route("/:module_id", routing::post(handlers::recv))
        .layer(Extension(Arc::new(registry)))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(args.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        args.port,
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
    pub module_dir: String,

    /// Log level
    #[arg(long = "log", default_value = "debug")]
    log_level: String,
}
