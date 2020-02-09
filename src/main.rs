extern crate xdg;
extern crate serde;
extern crate serde_yaml;
extern crate skim;

use skim::{Skim, SkimOptionsBuilder};
use serde::{Serialize, Deserialize};

use std::io::Cursor;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    commands: Vec<String>,
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

    let input = config.commands.join("\n");

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        println!("{}", item.get_output_text());
    }
}
