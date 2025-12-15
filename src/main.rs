mod cli_lib;
mod logger;
mod ping;

use clap::Parser;
use cli_lib::{Cli, Commands, OutputLevel};

use logger::logger::Logger;

//use dns_lookup::{lookup_addr, lookup_host};

fn main() {
    // let hostname = "google.com";
    // let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    // println!("IPs: {:?}", ips);
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Ping(args) => {
            //app_init(&args.verbosity);
            ping::ping(&cli, args);
        }
        Commands::Trace(args) => {
            println!("'trace' for {}, ttl is: {}", args.dest, args.ttl);
        }
    }
}

fn app_init(level: &OutputLevel) {
    println!("Log level is: {}", level);
    let l: Logger = Logger::new(level);

    l.test_all();
}
