use crate::error;

pub enum ArgOption {
    Single(String),
    Pair(String, String),
}

pub struct ArgParser {
    pub options: Vec<ArgOption>,
    pub arguments: Vec<String>,
}

impl ArgParser {
    pub fn new() -> Self {
        use std::env;

        let mut args = env::args();
        args.next();

        let mut options: Vec<String> = Vec::new();
        let mut arguments: Vec<String> = Vec::new();

        while let Some(mut argument) = args.next() {
            argument = argument.trim().to_owned();

            if argument.len() == 0 {
                error::error_and_quit(format!("empty arguments are disallowed").as_ref());
            }

            match argument.get(0..1).unwrap() {
                "-" => {
                    
                },
                _ =>
            }
        }
        ArgParser {
            options: Vec::new(),
            arguments: Vec::new(),
        }
    }
}
