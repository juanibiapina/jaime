extern crate xdg;
extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};

use std::io::Write;
use std::fs::File;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    commands: Vec<String>,
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("jaime").unwrap();
    let config_path = xdg_dirs.place_config_file("plugins.yaml").expect("cannot create config directory");

    let file = File::open(config_path).expect("cannot open config file");

    let config: Config = serde_yaml::from_reader(file).unwrap();

    let mut fzf_cmd = Command::new("sh")
        .arg("-c")
        .arg("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start fzf");

    {
        let stdin = fzf_cmd.stdin.as_mut().expect("failed to open stdin to fzf");
        for command in config.commands.iter() {
            stdin.write_all(command.as_bytes()).expect("failed to write to stdin");
            stdin.write_all("\n".as_bytes()).expect("failed to write to stdin");
        }
    }

    let output = fzf_cmd.wait_with_output().expect("failed to read stdout from fzf");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
