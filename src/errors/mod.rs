#[derive(Debug, Default)]
struct LoxError {
    error_msg: String,
    line_num: usize,
}
#[derive(Debug, Default)]
pub struct ErrorHandler {
    errors: Vec<LoxError>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { errors: Vec::new() }
    }
    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn report(&mut self, error_msg: String, line_num: usize) {
        let lox_error = LoxError {
            error_msg,
            line_num,
        };
        self.errors.push(lox_error);
    }

    pub fn display_errors(&self, source_code: &str) {
        // let source_lines: Vec<&str> = source_code.lines().collect();
        for err in &self.errors {
            eprintln!(
                "[line {}] Error: {}",
                err.line_num,
                err.error_msg // source_lines[err.line_num as usize - 1]
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_error_handler() {
        let source_code = "\
var a = 9;
var 8b = 8;
var c = a + b;
print(\"abc\");
var j = 094342;
";
        let mut error_handler = ErrorHandler::new();
        error_handler.report(String::from("This is a simple error"), 2);
        error_handler.report(String::from("This is error number 2"), 5);
        error_handler.display_errors(source_code);
    }
}
