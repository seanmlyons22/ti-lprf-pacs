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
        Some("install-deps") => install_deps(),
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
install-deps        Install the required dependencies for generating the SVDs and PACs and building the crates
"
    )
}

fn install_deps() -> Result<(), DynError> {
    // Create a from PAC name to target triple
    let mut map = std::collections::HashMap::new();
    map.insert("cc2340r5", "thumbv6m-none-eabi");

    // Check that rustup is installed and on path
    Command::new("rustup").args(["target", "list"]).output()?;
    // Check that cargo is installed and on path
    Command::new("cargo").args(["--version"]).output()?;

    // Install necessary target triple
    for (pac_name, target_triple) in map {
        println!("Installing dependencies for {}", pac_name);
        Command::new("rustup")
            .args(["target", "add", target_triple])
            .stdout(io::stdout())
            .stderr(io::stdout())
            .output()?;
    }

    Command::new("cargo")
        .args(["install", "svd2rust@0.33.1", "--locked"])
        .stdout(io::stdout())
        .stderr(io::stdout())
        .output()?;

    Command::new("cargo")
        .args(["install", "svd2rust@0.33.1", "--locked"])
        .stdout(io::stdout())
        .stderr(io::stdout())
        .output()?;

    Command::new("cargo")
        .args(["install", "form@0.12.1", "--locked"])
        .stdout(io::stdout())
        .stderr(io::stdout())
        .output()?;

    Command::new("cargo")
        .args([
            "install",
            "--git",
            "https://github.com/seanmlyons22/tixml2svd.git",
            "--rev",
            "e826132977a3356cbef343442de89cf16d2b985c",
            "--locked",
        ])
        .stdout(io::stdout())
        .stderr(io::stdout())
        .output()?;
    Ok(())
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
    if cfg!(windows) {
        python_executable = "python";
    }

    // Check that svd2rust is on path before beginning. To be overly pedanic other deps could be checked
    let result = Command::new("svd2rust").args(["--version"]).output();
    if result.is_err() {
        // return an error
        return Err(
            "svd2rust not found on path, consider running `cargo xtask install-deps`".into(),
        );
    }

    println!("Generating SVDs and PACs using xtask");
    println!("Python executable: {}", python_executable);

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
