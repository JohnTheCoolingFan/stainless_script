#[cfg(feature = "format-bincode")]
use bincode::deserialize_from as bincode_from_reader;
use clap::Parser;
use ron::de::from_reader as ron_from_reader;
#[cfg(feature = "format-json")]
use serde_json::from_reader as json_from_reader;
use stainless_script::{
    module::ModulePath,
    program::{Program, ProgramCollection}, Executor, stdlib::StdPlugin,
};
use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

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

fn format_from_filename(file_name: &str) -> ProgramFormat {
    if file_name.ends_with(".ron.ssc") {
        return ProgramFormat::Ron
    }
    #[cfg(feature = "format-json")]
    if file_name.ends_with(".json.ssc") {
        // These cfgs are a mess but it's a required workaround to make this compile
        return ProgramFormat::Json;
    }
    #[cfg(feature = "format-bincode")]
    if file_name.ends_with(".bin.ssc") {
        return ProgramFormat::Bincode;
    }
    panic!("Failed to determine program format based on file extension, please specify program format using --format")
}

fn read_program(path: &Path, format: &ProgramFormat) -> Program {
    let program_file = File::open(path).unwrap();
    match format {
        ProgramFormat::Ron => ron_from_reader(program_file).unwrap(),
        #[cfg(feature = "format-json")]
        ProgramFormat::Json => json_from_reader(program_file).unwrap(),
        #[cfg(feature = "format-bincode")]
        ProgramFormat::Bincode => bincode_from_reader(program_file).unwrap(),
    }
}

fn read_imports(program: &Program, programs: &mut ProgramCollection) {
    if let Some(imports) = &program.imports {
        for import in imports {
            let imported_program = read_import(import);
            read_imports(&imported_program, programs);
            programs
                .programs
                .insert(ModulePath::from_str(import).unwrap(), imported_program);
        }
    }
}

fn read_import(name: &str) -> Program {
    let path = ModulePath::from_str(name).unwrap();
    let fs_path = PathBuf::from(LINUX_LIB_PATH).join(PathBuf::from_iter(path.0.iter()));
    let mut candidates =
        glob::glob(&format!("{}/{}.*.ssc", fs_path.to_str().unwrap(), path.1)).unwrap();
    let program_path = candidates
        .next()
        .unwrap_or_else(|| panic!("Failed to find import for `{}`", name))
        .unwrap();
    let format = format_from_filename(program_path.file_name().unwrap().to_str().unwrap());
    read_program(&program_path, &format)
}

fn main() {
    let cli = Cli::parse();

    let program_format = cli.format.unwrap_or_else(|| {
        let file_name = cli.program.file_name().unwrap().to_str().unwrap();
        format_from_filename(file_name)
    });

    let main_program = read_program(&cli.program, &program_format);

    let mut programs = ProgramCollection::default();

    read_imports(&main_program, &mut programs);

    programs
        .programs
        .insert(ModulePath(vec![], "__main__".into()), main_program);

    let mut executor = Executor::default();
    // ADD PLUGINS HERE
    executor.load_plugin(StdPlugin);

    executor.load_programs(programs);

    executor.start_execution(true);
}
