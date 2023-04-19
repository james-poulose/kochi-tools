use clap::{Args, Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Pings a server
    Ping(PingArgs),

    /// Traces the route of a Ping.
    Trace(TraceArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
pub struct PingArgs {
    pub dest: String,

    #[arg(short, long, default_value_t = 116)]
    pub ttl: u8,
}

#[derive(Args)]
pub struct TraceArgs {
    pub dest: String,

    #[arg(short, long, default_value_t = 116)]
    pub ttl: u8,
}
