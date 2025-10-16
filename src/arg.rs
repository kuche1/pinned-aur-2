use clap::Parser; // cargo add clap --features derive

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of package to search for
    // #[arg(short, long)] // with this the argument is no longer positional
    package: String,
}

pub fn parse() -> String {
    let args = Args::parse();
    args.package
}
