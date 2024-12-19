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
        write!(f, "[Line - {} ] \n Error {} : {}", self.line, self.where_r, self.message)
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

    pub fn error_panic<T>(line: u32, where_r: &str, message: &str) -> Result<T> {
        panic!("[Line - {} ] \n Error {} : {}", line, where_r, message);
    }
}


// Thinking to do something like this, look I trying taking this from C#. It's this good ?
// With that problably I can implementing many exception, where will be some erros return.
// And for the runtime I can implement with the panic

// pub type Result<T, E: BaseException> = std::result::Result<T, E>;

// pub trait BaseException: fmt::Debug + fmt::Display {
//     fn error<T, E>(line: u32, where_r: &str, message: &str) -> Result<T, E>;

//     fn new(line: u32, where_r: &str, message: &str) -> Self;
// }

// #[derive(Debug, Clone)]
// pub struct Exception {
//     line: u32,
//     message: String,
//     where_r: String
// }

// impl BaseException for Exception {
//     fn new(line: u32, where_r: &str, message: &str) -> Self {
//         Self {
//             line: line,
//             message: message.to_string(),
//             where_r: where_r.to_string()
//         }
//     }

//     fn error<T, E>(line: u32, where_r: &str, message: &str) -> Result<T, Exception> {

//     }
// }

// impl BaseException for ExceptionRuntime {
//     fn new(line: u32, where_r: &str, message: &str) -> Self {
//         Self {
//             line: line,
//             message: message.to_string(),
//             where_r: where_r.to_string()
//         }
//     }

//     fn error<T, E>(line: u32, where_r: &str, message: &str) -> Result<T, Exception> {
        // panic!();
//     }
// }
