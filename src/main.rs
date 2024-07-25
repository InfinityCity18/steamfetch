mod app;
mod arg;
mod error;
mod glyphs;
mod print;

#[tokio::main]
async fn main() {
    let mut a = match arg::ArgParser::new() {
        Ok(a) => a,
        Err(e) => error::print_error_and_exit(e),
    };
    match a.run_command().await {
        Ok(_) => (),
        Err(e) => error::print_error_and_exit(e),
    };
}
