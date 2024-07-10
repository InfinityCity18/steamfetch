pub fn error_and_quit(msg: &str) -> ! {
    println!("steamfetch: {}", msg);
    std::process::exit(1);
}
