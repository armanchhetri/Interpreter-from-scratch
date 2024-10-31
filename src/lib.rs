pub mod errors;
pub mod scanner;
pub mod superiterator;
pub mod token;

use errors::ErrorHandler;
use scanner::Scanner;
// use token::{Token, TokenType};

pub fn tokenize(source_code: &str) {
    let error_handler = ErrorHandler::new();
    let mut scanner = Scanner::new(source_code, error_handler);
    let tokens = scanner.scan();

    for token in tokens {
        println!("{}", token.token_type);
    }
}
