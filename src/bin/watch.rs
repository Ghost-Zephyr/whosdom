use std::{
    vec::Vec,
    io};

use clap::Parser;

use whosdom::whois::{Watcher, get_servers};

/// Domain registry watcher
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Domain(s) to watch
    #[arg(short, long)]
    domains: Vec<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut args = Args::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting whosdomain watch");
    log::trace!("With args: {args:?}");
    if args.domains.is_empty() {
        log::error!("No domains specified, using \"example.com\"");
        args.domains.push(String::from("example.com"));
    }
    log::info!("Watchin' domains: {:?}", args.domains);
    let watcher = Watcher::new(get_servers().await.unwrap(), args.domains.clone());
    log::info!("\n{}", watcher.lookup(&args.domains[0]).await);
    //watcher.watch();
    Ok(())
}
