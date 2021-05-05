const JAVA_PATH: &str = "/usr/bin/java";
const MVN_PATH: &str = "/usr/bin/mvn";
const PARENT_PATH: &str = "../";
const SPIGOT_SERVER_PATH: &str = "../Tuinity-Server";
const DEV_SERVER_PATH: &str = "../dev-server";
const TEST_PLUGIN_PATH: &str = "../test-plugin";

use execute::Execute;
use project_root::{get_project_root};
use std::process::Command;
use std::path::{PathBuf};
use std::fs::{create_dir, metadata, copy, File};
use std::env::{set_current_dir, args, Args};
use std::io::Write;

/// This simple Rust app uses Maven CLI to compile
/// the API & Server, then starts the server automatically.
/// It is recommended to get a Rust integration for your IDE,
/// so it is easy for you to execute this app.

/// Let's go
fn main() {
    let root_dir: PathBuf = get_project_root().unwrap();
    let mut arg: Args = args();

    // Create the dev server path if it doesn't exist
    if !metadata(DEV_SERVER_PATH).is_ok() {
        println!("Creating {}", DEV_SERVER_PATH);
        create_dir(DEV_SERVER_PATH);
    }

    if arg.any(|a| a == "--recompile") {
        println!("Running `package` against the parent ({})", PARENT_PATH);
        set_current_dir(PARENT_PATH);
        run_command(
            Command::new(MVN_PATH)
                .arg("package")
        );

        // Go back
        set_current_dir(root_dir);
    }
    // Compile the parent

    // Copy the compiled plugin jar
    copy(
        format!("{}{}", TEST_PLUGIN_PATH, "target/test-plugin.jar"),
        format!("{}{}", DEV_SERVER_PATH, "plugins/test-plugin.jar")
    );

    // Set the current dir to the path of the dev server
    set_current_dir(DEV_SERVER_PATH);

    // Create eula.txt
    let eula = File::create("eula.txt");
    eula.unwrap().write_all(b"eula=true");

    // Start the server
    run_command(
        Command::new(JAVA_PATH)
            .arg("-jar")
            .arg(format!("{}{}", SPIGOT_SERVER_PATH, "/target/matherionity-1.16.5.jar"))
    );
}

fn run_command(command: &mut Command) {
    let output = command.execute_output().unwrap();

    // End the process
    println!("Process finished with exit code {}.", output.status.code().unwrap());
}