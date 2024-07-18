use crate::error::{ExitError, ExitResult, IntoResultExitError};

const PAIR_OPTIONS: &'static [&'static str] = &[
    "-t",
    "-token",
    "--t",
    "--token",
    "-l",
    "-lang",
    "--l",
    "--language",
];
const SINGLE_OPTIONS: &'static [&'static str] = &[
    "-h", "-help", "--h", "--help", "-f", "-font", "--f", "--font",
];

#[derive(Debug)]
pub enum ArgOption {
    Single(String),
    Pair(String, String),
}

#[derive(Debug)]
pub struct ArgParser {
    pub options: Vec<ArgOption>,
    pub arguments: Vec<String>,
    pub token: Option<String>,
}

impl ArgParser {
    pub fn new() -> ExitResult<'static, ArgParser> {
        use std::env;

        let mut args = env::args();
        let token = env::var("STEAM_TOKEN").ok();

        args.next();

        let mut options: Vec<ArgOption> = Vec::new();
        let mut arguments: Vec<String> = Vec::new();

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
                        let next_arg =
                            <std::option::Option<std::string::String> as IntoResultExitError<
                                '_,
                                std::string::String,
                                (),
                            >>::into_exit_error(args.next(), "")?;
                        options.push(ArgOption::Pair(argument, next_arg));
                    } else {
                        return Err(ExitError("empty arguments are disallowed"));
                    }
                }
                _ => arguments.push(argument),
            }
        }
        Ok(ArgParser {
            options,
            arguments,
            token,
        })
    }
}
