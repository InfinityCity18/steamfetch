use std::str::FromStr;

use crate::{
    app::print,
    error::{ExitError, ExitResult, IntoResultExitError},
    glyphs::{FancyFont, NoFancyFont},
};

const PAIR_OPTIONS: &'static [&'static str] =
    &["-t", "--token", "-l", "--language", "-o", "--offset"];
const SINGLE_OPTIONS: &'static [&'static str] =
    &["-h", "--help", "-f", "--font", "-v", "--version"];
const VERSION: &str = env!("CARGO_PKG_VERSION");

const DEFAULT_WIDTH: u16 = 51;

#[derive(Debug)]
pub enum ArgOption {
    Single(String),
    Pair(String, String),
}

#[derive(Debug)]
pub struct ArgParser {
    options: Vec<ArgOption>,
    commands: Vec<String>,
    token: Option<String>,
    language: Option<String>,
    nofont: bool,
    offset: u16,
}

impl ArgParser {
    pub fn new() -> ExitResult<'static, ArgParser> {
        use std::env;

        let mut args = env::args();
        let token = env::var("STEAM_TOKEN").ok();

        args.next();

        let mut options: Vec<ArgOption> = Vec::new();
        let mut commands: Vec<String> = Vec::new();

        while let Some(mut argument) = args.next() {
            argument = argument.trim().to_owned();

            if argument.len() == 0 {
                return Err(ExitError("empty arguments are disallowed"));
            }

            match argument.get(0..1).unwrap() {
                "-" => {
                    if SINGLE_OPTIONS.contains(&argument.as_str()) {
                        options.push(ArgOption::Single(argument));
                    } else if PAIR_OPTIONS.contains(&argument.as_str()) {
                        let next_arg = args
                            .next()
                            .into_exit_error("argument was not supplied for one of options")?;
                        options.push(ArgOption::Pair(argument, next_arg));
                    } else {
                        return Err(ExitError("wrong option provided"));
                    }
                }
                _ => commands.push(argument),
            }
        }
        Ok(ArgParser {
            options,
            commands,
            token,
            language: None,
            nofont: false,
            offset: 2,
        })
    }

    fn process_options(&mut self) -> ExitResult<'static, ()> {
        for option in &self.options {
            match option {
                ArgOption::Single(a) => match a.as_ref() {
                    "-h" | "--help" => {
                        print!("{}", HELP);
                        std::process::exit(0);
                    }
                    "-v" | "--version" => {
                        println!("steamfetch {}", VERSION);
                        std::process::exit(0);
                    }
                    "-f" | "--font" => self.nofont = true,
                    _ => unreachable!(),
                },
                ArgOption::Pair(a, b) => match a.as_ref() {
                    "-t" | "--token" => self.token = Some(b.clone()),
                    "-l" | "--language" => self.language = Some(b.clone()),
                    "-o" | "--offset" => {
                        self.offset = b
                            .parse::<u16>()
                            .into_exit_error("number for offset could not be parsed")?
                    }
                    _ => unreachable!(),
                },
            };
        }
        Ok(())
    }

    #[allow(unreachable_code)]
    pub fn run_command(&mut self) -> ExitResult<'static, ()> {
        self.process_options()?;

        let command = self
            .commands
            .get(0)
            .into_exit_error("no commands provided")?;

        match command.as_ref() {
            "app" | _ => {
                let mut iter = self.commands.iter();
                let first_arg = iter.next().to_owned().unwrap();
                let mut app_name;
                if first_arg == "app" {
                    app_name = String::new();
                } else {
                    app_name = String::from_str(first_arg).unwrap();
                }
                while let Some(comm) = iter.next() {
                    app_name += " ";
                    app_name += comm;
                }
                if first_arg == "app" {
                    app_name.remove(0);
                }

                let fg_mod =
                    crate::print::constants::TEXT_COLOR.to_string() + crate::print::constants::BOLD;
                let bg_color = crate::print::constants::NONE;

                let width = DEFAULT_WIDTH;
                let frame_height = (width * 10) / 46;

                if self.nofont {
                    crate::app::print::<NoFancyFont>(
                        &app_name,
                        self.language.as_ref().unwrap_or(&"".to_string()),
                        width,
                        frame_height,
                        self.offset,
                        &fg_mod,
                        bg_color,
                    )?;
                } else {
                    crate::app::print::<FancyFont>(
                        &app_name,
                        self.language.as_ref().unwrap_or(&"".to_string()),
                        width,
                        frame_height,
                        self.offset,
                        &fg_mod,
                        bg_color,
                    )?;
                }
            }
        }

        Ok(())
    }
}

const HELP: &str = "\
Usage: steamfetch [options] [commands]

Options:
    -h, --help                  Print help and exit
    -v, --version               Print version and exit
    -f, --font                  Fallback to not using nerd font for display
    -t, --token <TOKEN>         Provide Steam API token, not implemented yet
    -l, --language <LANGUAGE>   Change language of some elements, provided by Steam; e.g. app description
    -o, --offset <OFFSET>       Inserts amount of whitespaces before display, i.e. shifts the whole thing to right. Default is 2

Commands:
    ...     <APP TITLE>         Show app information, ... means that you can supply APP TITLE instantly; every argument after merges into one
    app     <APP TITLE>         As the one above, just that the first argument is not included in search
";
