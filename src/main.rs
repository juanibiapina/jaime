#[macro_use] extern crate failure;
extern crate rustyline;
extern crate xdg;
extern crate serde;
extern crate serde_yaml;
extern crate skim;

use failure::{Error, ResultExt};
use skim::{Skim, SkimOptionsBuilder, SkimItemReader};
use serde::{Serialize, Deserialize};
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Cursor;
use std::fs::File;
use std::process;
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
        preview: Option<String>,
        command: String,
    },
    FreeText {
        command: String,
    },
}

fn display_selector(input: String, preview: Option<&str>) -> Result<Option<String>, Error> {
    let options = SkimOptionsBuilder::default()
        .multi(false)
        .ansi(true)
        .preview(preview)
        .build()
        .map_err(|err| format_err!("{}", err))?;

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    Ok(selected_items
        .iter()
        .next()
        .map(|selected| selected.output().to_string()))
}

fn run_shell(context: &Context, cmd: &str) -> Result<(), Error> {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .env("JAIME_CACHE_DIR", &context.cache_directory)
        .status()?;

    Ok(())
}

fn run_shell_command_for_output(context: &Context, cmd: &str) -> Result<String, Error> {
    Ok(std::str::from_utf8(Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .env("JAIME_CACHE_DIR", &context.cache_directory)
        .output()?
        .stdout
        .as_slice())?.to_owned())
}

fn run_widget(context: &Context, widget: &Widget) -> Result<(), Error> {
    match widget {
        Widget::Command { command } => run_shell(context, command),
        Widget::Select { options } => {
            let input = options.keys().map(|k| k.as_ref()).collect::<Vec<&str>>().join("\n");
            let selected_command = display_selector(input, None)?;

            if let Some(selected_command) = selected_command {
                match options.get(&selected_command) {
                    Some(widget) => { run_widget(context, widget) },
                    None => { Ok(()) },
                }
            } else {
                Ok(())
            }
        },
        Widget::DynamicSelect { arguments, command, preview } => {
            let mut args: Vec<String> = Vec::new();

            for (index, argument) in arguments.iter().enumerate() {
                let mut result = argument.clone();

                for i in 0..index {
                    result = result.replace(&format!("{{{}}}", i), &args[i]);
                }

                let output = run_shell_command_for_output(context, &result)?;

                let selected_command = display_selector(output, preview.as_ref().map(|s| s.as_ref()))?;

                if let Some(selected_command) = selected_command {
                    args.push(selected_command);
                } else {
                    return Ok(());
                }
            }

            let mut cmd = command.clone();

            for (index, arg) in args.iter().enumerate() {
                cmd = cmd.replace(&format!("{{{}}}", index), arg);
            }

            run_shell(context, &cmd)
        },
        Widget::FreeText { command } => {
            let mut rl = Editor::<()>::new();

            let line = rl.readline("> ");
            match line {
                Ok(line) => {
                    let cmd = command.replace("{}", &line);
                    run_shell(context, &cmd)
                },
                Err(ReadlineError::Interrupted) => {
                    Ok(())
                },
                Err(ReadlineError::Eof) => {
                    Ok(())
                },
                Err(err) => {
                    Err(err)?
                }
            }
        },
    }
}

fn actual_main() -> Result<(), Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("jaime")?;
    let config_path = xdg_dirs.place_config_file("config.yml")?;

    let file = File::open(config_path).context("Couldn't read config file")?;

    let config: Config = serde_yaml::from_reader(file)?;

    let input = config.widgets.keys().map(|k| k.as_ref()).collect::<Vec<&str>>().join("\n");

    let selected_command = display_selector(input, None)?;

    if let Some(selected_command) = selected_command {
        let widget = config.widgets.get(&selected_command).unwrap();

        let context = Context {
            cache_directory: xdg_dirs.create_cache_directory("cache")?,
        };

        run_widget(&context, widget)?;
    }

    Ok(())
}

fn main() {
    match actual_main() {
        Ok(()) => {},
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        },
    }
}
