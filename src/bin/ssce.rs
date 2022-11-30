use clap::Parser;

const LINUX_LIB_PATH: &str = "/usr/lib/stainless_script/";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    program: String
}

fn main() {
    let cli = Cli::parse();
}
