use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GlrnvimError {
    message: String,
}

impl GlrnvimError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GlrnvimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GlrnvimError {
    fn description(&self) -> &str {
        "glrnvim errors"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
