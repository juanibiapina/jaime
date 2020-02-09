extern crate xdg;
extern crate serde;
extern crate serde_yaml;
extern crate skim;

use skim::{Skim, SkimOptionsBuilder};
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::io::Cursor;
use std::fs::File;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    widgets: HashMap<String, Widget>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Widget {
    Command {
        command: String,
    },
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("jaime").unwrap();
    let config_path = xdg_dirs.place_config_file("config.yaml").expect("cannot create config directory");

    let file = File::open(config_path).expect("cannot open config file");

    let config: Config = serde_yaml::from_reader(file).unwrap();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let input = config.widgets.keys().map(|k| k.as_ref()).collect::<Vec<&str>>().join("\n");

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected = selected_items.iter().next().unwrap();
    let selected_command: String = selected.get_output_text().to_string();

    let widget = config.widgets.get(&selected_command).unwrap();

    match widget {
        Widget::Command { command } => {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .unwrap();
            },
    }
}
