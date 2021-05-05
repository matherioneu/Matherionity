const JAVA_PATH: &str = "/usr/bin/java";
const TARGET_JAR_PATH: &str = "../Tuinity-Server/target/matherionity-1.16.5.jar";
const DEV_SERVER_PATH: &str = "../dev-server";

extern crate execute;

use std::process::Command;

use execute::Execute;
use std::path::Path;
use std::fs::{create_dir, metadata, File};
use std::env::{set_current_dir};
use std::io::Write;

fn main() {
    // Create the dev server path if it doesn't exist
    if !metadata(DEV_SERVER_PATH).is_ok() {
        create_dir(DEV_SERVER_PATH);
    }

    // Set the current dir to the path of the dev server
    set_current_dir(DEV_SERVER_PATH);

    // Create eula.txt
    let mut eula = File::create("eula.txt");
    eula.unwrap().write_all(b"eula=true");

    // Start the server
    let mut command = Command::new(JAVA_PATH);

    command.arg("-jar");
    command.arg(TARGET_JAR_PATH);

    let output = command.execute_output().unwrap();

    // End the process
    println!("Process finished with exit code {}.", output.status.code().unwrap());
}