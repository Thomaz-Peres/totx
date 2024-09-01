pub struct Exception;

impl Exception {
    pub fn error(line: u32, message: &str) {
        Self::report(line, "", message)
    }

    fn report(line: u32, where_r: &str, message: &str) {
        panic!("[Line - {line} ] \n Error  {where_r} + : {message}");
    }
}
