pub struct Exception;

impl Exception {
    pub fn error(line: u32, message: &str) -> Self {
        Self::report(line, "", message)
    }

    fn report(line: u32, where_r: &str, message: &str) -> Self {
        panic!("[Line - {line} ] \n Error  {where_r} + : {message}");
    }
}
