extern crate xdg;
extern crate serde;
extern crate serde_yaml;
extern crate skim;

use skim::{Skim, SkimOptionsBuilder};
use serde::{Serialize, Deserialize};

use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Cursor;
use std::fs::File;
use std::process::Command;

struct Context {
    cache_directory: PathBuf,
}

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
    Select {
        options: HashMap<String, Widget>,
    },
    DynamicSelect {
        arguments: Vec<String>,
        template: String,
    },
}

fn display_selector(input: String) -> String {
    let options = SkimOptionsBuilder::default()
        .multi(false)
        .ansi(true)
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected = selected_items.iter().next().unwrap();

    selected.get_output_text().to_string()
}

fn run_shell(context: &Context, cmd: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .env("JAIME_CACHE_DIR", &context.cache_directory)
        .status()
        .unwrap();
}

fn run_shell_command_for_output(context: &Context, cmd: &str) -> String {
    std::str::from_utf8(Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .env("JAIME_CACHE_DIR", &context.cache_directory)
        .output()
        .unwrap()
        .stdout
        .as_slice()).unwrap().to_owned()
}

fn run_widget(context: &Context, widget: &Widget) {
    match widget {
        Widget::Command { command } => run_shell(context, command),
        Widget::Select { options } => {
            let input = options.keys().map(|k| k.as_ref()).collect::<Vec<&str>>().join("\n");
            let selected_command = display_selector(input);

            let widget = options.get(&selected_command).unwrap();

            run_widget(context, widget);
        },
        Widget::DynamicSelect { arguments, template } => {
            let mut args: Vec<String> = Vec::new();

            for (index, argument) in arguments.iter().enumerate() {
                let mut result = argument.clone();

                for i in 0..index {
                    result = result.replace(&format!("{{{}}}", i), &args[i]);
                }

                let output = run_shell_command_for_output(context, &result).to_owned();

                args.push(display_selector(output));
            }

            let mut cmd = template.clone();

            for (index, arg) in args.iter().enumerate() {
                cmd = cmd.replace(&format!("{{{}}}", index), arg);
            }

            run_shell(context, &cmd);
        },
    }
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("jaime").unwrap();
    let config_path = xdg_dirs.place_config_file("config.yml").expect("cannot create config directory");

    let file = File::open(config_path).expect("cannot open config file");

    let config: Config = serde_yaml::from_reader(file).unwrap();

    let input = config.widgets.keys().map(|k| k.as_ref()).collect::<Vec<&str>>().join("\n");

    let selected_command = display_selector(input);

    let widget = config.widgets.get(&selected_command).unwrap();

    let context = Context {
        cache_directory: xdg_dirs.create_cache_directory("cache").unwrap(),
    };

    run_widget(&context, widget);
}
