use execute::Execute;
use std::process::Command;

pub struct Cargo;

impl Cargo {
    pub fn run(args: Vec<&str>) {
        let mut command = Command::new("cargo");

        for arg in args {
            command.arg(arg);
        }

        let output = command.execute_output().unwrap();
    }
}
