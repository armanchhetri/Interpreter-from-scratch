pub mod errors;
pub mod scanner;
pub mod superiterator;
pub mod token;

use std::process;

use errors::ErrorHandler;
use scanner::Scanner;
// use token::{Token, TokenType};

pub fn tokenize(source_code: &str) {
    let mut error_handler = ErrorHandler::new();
    let mut scanner = Scanner::new(source_code, &mut error_handler);
    let tokens = scanner.scan_source();

    error_handler.display_errors(source_code);
    for token in tokens {
        println!("{}", token.token_type);
    }
    if error_handler.has_error() {
        process::exit(65);
    }
}
