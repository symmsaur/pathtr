use std::env;

pub fn parse() -> Result<Args, ArgParseError> {
    let mut args = Args::default();
    let mut argv = env::args();
    argv.next();
    for arg in argv {
        if arg == "-p" {
            args.preview = true;
        } else {
            return Err(ArgParseError {
                msg: format!("Unknown arg '{}'", arg),
            });
        }
    }
    Ok(args)
}

#[derive(Default)]
pub struct Args {
    pub preview: bool,
}

#[derive(Debug)]
pub struct ArgParseError {
    pub msg: String,
}
