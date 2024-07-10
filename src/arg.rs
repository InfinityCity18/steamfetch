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

        let arguments = env::args();
        arguments.next();

        while let Some(argument) = arguments.next() {}
    }
}
