use clap::{arg, command, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    lists: bool,

    #[arg(short, long, default_value = "Default")]
    select: String,

    #[arg(short, long)]
    new: String,

    #[command(subcommand)]
    subcmd: SubCommands,

}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Add,
    Remove,
}

fn main() {
    let args = Args::parse();

    if args.lists {
        println!("Hello, world!");
    }
}
