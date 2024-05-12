use clap::Parser;
use std::path::Path;

use pycruft::remove_bytecode;

#[derive(Parser)]
struct Args {
    directory: String,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    safe: bool,
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    remove_bytecode(dir, Some(args.verbose), Some(args.safe));
}
