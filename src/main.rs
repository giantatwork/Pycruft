use clap::Parser;
use std::path::Path;

use pycruft::start;

#[derive(Parser, Debug, Default)]
#[clap(
    author = "Giant at Work",
    version,
    about = "Pycruft is a Python bytecode cleaner written in Rust"
)]
struct Args {
    directory: String,
    /// Directory to start searching
    #[clap(short, long)]
    /// Verbose mode
    verbose: bool,
    #[clap(short, long)]
    /// Ask for confirmation
    confirm: bool,
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    start(dir, Some(args.confirm), Some(args.verbose));
}
