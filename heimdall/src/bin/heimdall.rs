
#[macro_use]
extern crate rocket;

use clap::Parser;
use heimdall::handlers;
use heimdall::registry::Registry;
use heimdall::resolver::disk::DiskResolver;

#[launch]
async fn rocket() -> _ {
    let args = Args::parse();

    let resolver = DiskResolver::new(args.module_dir);
    let registry = Registry::new(Box::new(resolver), args.max_cached_modules);

    let mut config = rocket::config::Config::debug_default();
    config.port = args.port;

    rocket::custom(config)
        .manage(registry)
        .mount("/", routes![handlers::recv])
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
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
