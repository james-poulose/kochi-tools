mod cli_lib;
mod ping;

use clap::Parser;
use cli_lib::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Ping(args) => {
            println!("'ping' for {}, ttl is: {}", args.dest, args.ttl);
            ping::ping(cli);
        }
        Commands::Trace(args) => {
            println!("'trace' for {}, ttl is: {}", args.dest, args.ttl);
        }
    }
}
