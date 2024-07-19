pub fn print_error_and_exit(err: ExitError) -> ! {
    eprintln!("steamfetch : {}", err.0);
    std::process::exit(1);
}

pub type ExitResult<'a, T> = Result<T, ExitError<'a>>;

#[derive(Debug)]
pub struct ExitError<'a>(pub &'a str); //change to String

pub trait IntoResultExitError<'a, T> {
    fn into_exit_error(self, msg: &'a str) -> Result<T, ExitError<'a>>;
}

impl<'a, T, E> IntoResultExitError<'a, T> for Result<T, E> {
    fn into_exit_error(self, msg: &'a str) -> Result<T, ExitError<'a>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(ExitError(msg)),
        }
    }
}

impl<'a, T> IntoResultExitError<'a, T> for Option<T> {
    fn into_exit_error(self, msg: &'a str) -> Result<T, ExitError<'a>> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(ExitError(msg)),
        }
    }
}
