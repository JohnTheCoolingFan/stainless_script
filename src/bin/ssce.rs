use clap::Parser;
use stainless_script::program::Program;
use std::{path::PathBuf, fs::File};
#[cfg(feature = "format-json")]
use serde_json::from_reader as json_from_reader;
#[cfg(feature = "format-bincode")]
use bincode::deserialize_from as bincode_from_reader;
use ron::de::from_reader as ron_from_reader;

const LINUX_LIB_PATH: &str = "/usr/lib/stainless_script/";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    program: PathBuf,

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
            _ => panic!("Invalid format: {}", s),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let program_format = cli.format.unwrap_or_else(|| {
        let file_name = cli.program.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".ron.ssc") {
            ProgramFormat::Ron
        } else if cfg!(feature = "format-json") && file_name.ends_with(".json.ssc") {
            ProgramFormat::Json
        } else if cfg!(feature = "format-bincode") && file_name.ends_with(".bin.ssc") {
            ProgramFormat::Bincode
        } else {
            panic!("Failed to determine program format based on file extension, please specify program format using --format")
        }
    });

    let program_file = File::open(&cli.program).unwrap();
    let program: Program = match program_format {
        ProgramFormat::Ron => ron_from_reader(program_file).unwrap(),
        #[cfg(feature = "format-json")]
        ProgramFormat::Json => json_from_reader(program_file).unwrap(),
        #[cfg(feature = "format-bincode")]
        ProgramFormat::Bincode => bincode_from_reader(program_file).unwrap(),
    };
}
