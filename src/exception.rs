use core::fmt;

#[derive(Debug, Clone)]
pub struct Exception {
    line: u32,
    message: String,
    where_r: String
}

pub type Result<T> = std::result::Result<T, Exception>;

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Line - {} ] \n Error {} + : {}", self.line, self.where_r, self.message)
    }
}

impl Exception {
    pub fn new(line: u32, where_r: &str, message: &str) -> Self {
        Self {
            line: line,
            message: message.to_string(),
            where_r: where_r.to_string()
        }
    }

    pub fn error<T>(line: u32, where_r: &str, message: &str) -> Result<T> {
        Err(Exception::new(line, where_r, message))
    }
}
