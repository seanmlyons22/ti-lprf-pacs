use std::process::Command;
use std::{env, io, path::Path};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("generate") => generate_svds_and_pacs(),
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    eprintln!(
        "Tasks:

generate            Generate the SVDs and PACs from the XML in `input/devices`
"
    )
}

fn generate_svds_and_pacs() -> Result<(), DynError> {
    // Get the directory the xtask folder
    let xtask_folder = env::var("CARGO_MANIFEST_DIR").unwrap();
    // The root of the repo is the parent of the xtask folder, this is assumed
    let root_path = Path::new(&xtask_folder).parent().unwrap();
    // Change the to the top level dir of the project
    env::set_current_dir(root_path).unwrap();

    // Set python executable based on OS
    let mut python_executable = "python3";
    if !cfg!(windows) {
        python_executable = "python";
    }

    // Run the python script to generate the SVDs and PACs based on CCS input
    // Redirect stdout and stderr to stdout
    Command::new(python_executable)
        .args([
            "tools/generate_pacs.py",
            "input/devices",
            "-osvds",
            "svds",
            "-opacs",
            "pacs",
        ])
        .stdout(io::stdout())
        .stderr(io::stdout())
        .output()?;
    Ok(())
}
