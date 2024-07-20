mod app;
mod arg;
mod error;
mod glyphs;
mod print;

fn main() {
    let mut a = match arg::ArgParser::new() {
        Ok(a) => a,
        Err(e) => error::print_error_and_exit(e),
    };
    match a.run_command() {
        Ok(_) => (),
        Err(e) => error::print_error_and_exit(e),
    };
}
