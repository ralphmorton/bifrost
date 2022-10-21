use axum::extract::Extension;
use axum::handler::Handler;
use axum::Router;
use axum::routing;
use axum::Server;
use clap::Parser;
use heimdall::handlers;
use heimdall::registry::Registry;
use heimdall::store::disk::DiskStore;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::auth::RequireAuthorizationLayer;
use tower_http::cors::CorsLayer;
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

    let auth_layer = RequireAuthorizationLayer::bearer(args.api_key.as_str());

    let handler_store = (handlers::store).layer(&auth_layer);
    let handler_delete = (handlers::delete).layer(&auth_layer);

    let app = Router::new()
        .route("/store/:module_id", routing::post(handler_store))
        .route("/delete/:module_id", routing::delete(handler_delete))
        .route("/execute/:module_id", routing::post(handlers::recv))
        .layer(Extension(Arc::new(registry)))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(CorsLayer::permissive());

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

    // API key for authenticated routes
    #[arg(long = "key")]
    pub api_key: String,

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
