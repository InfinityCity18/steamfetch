pub fn error_and_quit(msg: &str) -> ! {
    eprintln!("steamfetch: {}", msg);
    std::process::exit(1);
}
