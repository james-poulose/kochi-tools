mod cli_lib;
mod logger;
mod ping;

use clap::Parser;
use cli_lib::{Cli, Commands};
use windows::core::Result;

//use dns_lookup::{lookup_addr, lookup_host};

fn main() -> Result<()> {
    // let hostname = "google.com";
    // let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    // println!("IPs: {:?}", ips);
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Ping(args) => {
            let _ = ping::ping(args);
        }
        Commands::Trace(args) => {
            println!("'trace' for {}, ttl is: {}", args.dest, args.ttl);
        }
    }
    Ok(())
}
