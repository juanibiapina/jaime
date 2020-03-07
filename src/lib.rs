#[macro_use] extern crate failure;

use failure::Error;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use serde::{Serialize, Deserialize};
use skim::{Skim, SkimOptionsBuilder, SkimItemReader};

use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command;

pub struct Context {
    pub cache_directory: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub widgets: HashMap<String, Widget>,
}

impl Config {
    pub fn into_widget(self) -> Widget {
        Widget::Select {
            options: self.widgets,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Widget {
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


impl Widget {
    pub fn run(&self, context: &Context) -> Result<(), Error> {
        match self {
            Widget::Command { command } => run_shell(context, command),
            Widget::Select { options } => {
                let input = options.keys().map(|k| k.as_ref()).collect::<Vec<&str>>().join("\n");
                let selected_command = display_selector(input, None)?;

                if let Some(selected_command) = selected_command {
                    match options.get(&selected_command) {
                        Some(widget) => { widget.run(context) },
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
}
