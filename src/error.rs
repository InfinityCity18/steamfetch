pub fn print_and_exit(msg: &str) {
    eprintln!("steamfetch : {msg}");
    std::process::exit(1);
}

pub type ExitResult<'a, T> = Result<T, ExitError<'a>>;

pub struct ExitError<'a>(&'a str);

pub trait IntoResultExitError<'a, T, E> {
    fn into_exit_error(self, msg: &'a str) -> Result<T, ExitError<'a>>;
}

impl<'a, T, E> IntoResultExitError<'a, T, E> for Result<T, E> {
    fn into_exit_error(self, msg: &'a str) -> Result<T, ExitError<'a>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(ExitError(msg)),
        }
    }
}
