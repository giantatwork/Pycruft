use clap::Parser;
use std::path::Path;

use pycruft::start;

#[derive(Parser, Debug, Default)]
#[clap(
    author = "Giant at Work",
    version,
    about = "Pycruft is a Python __pycache__ cleaner written in Rust"
)]
struct Args {
    /// Directory to start searching
    directory: String,
    #[clap(short, long)]
    /// Skip confirmation
    skip_confirmation: bool,
    #[clap(short, long)]
    /// Verbose mode
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    start(dir, Some(args.skip_confirmation), Some(args.verbose));
}
