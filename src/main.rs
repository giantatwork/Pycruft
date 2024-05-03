use clap::Parser;
use std::path::Path;

use pycruft::remove_bytecode;

#[derive(Parser)]
struct Args {
    directory: String,
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    remove_bytecode(dir, args.verbose);
}
