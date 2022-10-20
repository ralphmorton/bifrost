
#[macro_use]
extern crate rocket;

use clap::Parser;
use heimdall::handlers;
use heimdall::registry::Registry;

#[launch]
async fn rocket() -> _ {
    let args = Args::parse();

    let registry = Registry::new(args.dir, args.max_cached_modules);

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
    #[arg(short, long)]
    pub port: u16,
    /// Module directory
    #[arg(short, long)]
    pub dir: String,
    /// Max cached modules
    #[arg(short, long)]
    pub max_cached_modules: u64
}
