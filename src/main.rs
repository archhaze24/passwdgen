use clap::Parser;
use passwdgen::{start, Args};

fn main() {
    let args = Args::parse();

    start(args);
}
