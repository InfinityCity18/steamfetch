use crate::error;

const PAIR_OPTIONS: &'static [&'static str] = &["-t", "-token", "--t", "--token"];
const SINGLE_OPTIONS: &'static [&'static str] = &["-h", "-help", "--h", "--help"];

#[derive(Debug)]
pub enum ArgOption {
    Single(String),
    Pair(String, String),
}

#[derive(Debug)]
pub struct ArgParser {
    pub options: Vec<ArgOption>,
    pub arguments: Vec<String>,
}

impl ArgParser {
    pub fn new() -> Self {
        use std::env;

        let mut args = env::args();
        args.next();

        let mut options: Vec<ArgOption> = Vec::new();
        let mut arguments: Vec<String> = Vec::new();

        while let Some(mut argument) = args.next() {
            argument = argument.trim().to_owned();

            if argument.len() == 0 {
                error::error_and_quit(format!("empty arguments are disallowed").as_ref());
            }

            match argument.get(0..1).unwrap() {
                "-" => {
                    if SINGLE_OPTIONS.contains(&argument.as_str()) {
                        options.push(ArgOption::Single(argument));
                    } else if PAIR_OPTIONS.contains(&argument.as_str()) {
                        let next_arg = args.next().unwrap_or_else(|| {
                            error::error_and_quit(
                                format!("argument not provided for \"{}\" option", argument)
                                    .as_ref(),
                            )
                        });
                        options.push(ArgOption::Pair(argument, next_arg));
                    } else {
                        error::error_and_quit(format!("no such option: {}", argument).as_ref());
                    }
                }
                _ => arguments.push(argument),
            }
        }
        ArgParser { options, arguments }
    }
}
