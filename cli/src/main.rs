use clap::Parser;

pub fn main() {
    let args = cli::Args::parse();

    cli::main(args)
}
