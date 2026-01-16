use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tuffctl")]
#[command(about = "TUFF-OS Management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Status,
    Commit,
    Truncate,
    Cleanup,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => println!("Initialize..."),
        _ => println!("Not implemented yet"),
    }
}
