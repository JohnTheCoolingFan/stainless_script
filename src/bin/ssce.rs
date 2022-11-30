use clap::Parser;

const LINUX_LIB_PATH: &str = "/usr/lib/stainless_script/";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    program: String,

    #[arg(short, long, value_enum)]
    format: Option<ProgramFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
enum ProgramFormat {
    Ron,
    #[cfg(feature = "format-json")]
    Json,
    #[cfg(feature = "format-bincode")]
    Bincode,
}

impl From<String> for ProgramFormat {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ron" => Self::Ron,
            #[cfg(feature = "format-json")]
            "json" => Self::Json,
            #[cfg(feature = "format-bincode")]
            "bincode" => Self::Bincode,
            _ => panic!("Invalid format: {}", s)
        }
    }
}

fn main() {
    let cli = Cli::parse();
}
