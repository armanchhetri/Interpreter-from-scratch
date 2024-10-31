use crate::errors::ErrorHandler;
use crate::superiterator::SuperIterator;
use crate::token::number::Number;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source_code: Vec<char>,
    curr_idx: usize,
    curr_line: usize,
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &str, error_handler: &'a mut ErrorHandler) -> Self {
        Scanner {
            source_code: source_code.chars().collect(),
            curr_idx: 0,
            curr_line: 1,
            error_handler,
        }
    }

    pub fn scan_source(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.next() {
            if c == '\n' {
                self.curr_line += 1;
                continue;
            }
            let token_type: TokenType = match c {
                '=' => {
                    if self.is_next('=') {
                        self.next();
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '!' => {
                    if self.is_next('=') {
                        self.next();
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '<' => {
                    if self.is_next('=') {
                        self.next();
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if self.is_next('=') {
                        self.next();
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '/' => {
                    if self.is_next('/') {
                        self.read_until('\n');
                        self.curr_line += 1;
                        continue;
                    } else {
                        TokenType::Slash
                    }
                }
                '"' => self.get_string_token(),

                other_char => {
                    if other_char.is_whitespace() {
                        continue;
                    }
                    let mut token = TokenType::None;
                    // check if single character is part of language
                    if let Some(t) = self.get_from_factory(other_char.to_string()) {
                        token = t;
                    } else {
                        // handle other cases
                        if other_char.is_numeric() {
                            token = self.get_number().unwrap_or(TokenType::None);
                        } else if other_char.is_alphanumeric() {
                            // its either a keyword or an identifier
                            token = self.get_keyword_or_identifier();
                        }
                    }
                    token
                }
            };
            if token_type != TokenType::None {
                tokens.push(Token::new(token_type, self.curr_line));
            } else {
                self.error_handler
                    .report(format!("Unexpected character: {}", c), self.curr_line);
                // println!("Got none token type");
            }
        }
        tokens.push(Token::new(TokenType::EOF, self.curr_line));
        tokens
    }

    fn get_string_token(&mut self) -> TokenType {
        let mut string_str = String::new();
        for c in &mut *self {
            // while let Some(c) = self.next() {
            if c == '"' {
                return TokenType::String(string_str);
            }
            string_str.push(c);
        }
        self.error_handler
            .report(String::from("Non-terminated string value"), self.curr_line);
        TokenType::None
    }

    fn get_keyword_or_identifier(&mut self) -> TokenType {
        self.prev();
        let mut keyword_str = String::new();
        while let Some(k) = self.peek() {
            if k.is_alphanumeric() {
                keyword_str.push(k);
                self.next();
            } else {
                break;
            }
        }

        match keyword_str.parse::<TokenType>() {
            Ok(kw) => kw,
            Err(_) => TokenType::Identifier(keyword_str),
        }
    }

    fn get_number(&mut self) -> Option<TokenType> {
        let mut number_str = String::new();
        self.prev(); // go back one step
        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                number_str.push(c);
                self.next();
            } else {
                break;
            }
        }
        match number_str.parse::<Number>() {
            Ok(n) => Some(TokenType::Number(n)),
            Err(_) => {
                self.error_handler
                    .report(String::from("impossible to parse a number"), self.curr_line);
                None
            }
        }
    }
    fn read_until(&mut self, end: char) {
        for c in self {
            if c == end {
                break;
            }
        }
    }

    fn is_next(&self, c: char) -> bool {
        if let Some(next) = self.peek() {
            next == c
        } else {
            false
        }
    }
    fn get_from_factory(&self, key: String) -> Option<TokenType> {
        key.parse::<TokenType>().ok()
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_idx < self.source_code.len() {
            let curr_char = self.source_code[self.curr_idx];
            self.curr_idx += 1;
            Some(curr_char)
        } else {
            None
        }
    }
}

impl<'a> SuperIterator for Scanner<'a> {
    fn prev(&mut self) -> Option<Self::Item> {
        if self.curr_idx > 0 {
            let expected_idx = self.curr_idx - 1;
            let curr_char = self.source_code[expected_idx];
            self.curr_idx = expected_idx;
            Some(curr_char)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Self::Item> {
        if self.curr_idx < self.source_code.len() {
            Some(self.source_code[self.curr_idx])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<Self::Item> {
        let expected_idx = self.curr_idx + 1;
        if expected_idx < self.source_code.len() {
            Some(self.source_code[expected_idx])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespaces() {
        let mut error_handler = ErrorHandler::new();
        let source_code = "\
(        
                ) ";
        let mut scanner = Scanner::new(source_code, &mut error_handler);
        let expected = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::RightParen, 2),
            Token::new(TokenType::EOF, 2),
        ];
        let got = scanner.scan_source();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_slash() {
        let mut error_handler = ErrorHandler::new();
        let source_code = "()// Comment";
        let mut scanner = Scanner::new(source_code, &mut error_handler);
        let expected = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::RightParen, 1),
            Token::new(TokenType::EOF, 1),
        ];
        let got = scanner.scan_source();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_equal_equal() {
        let mut error_handler = ErrorHandler::new();
        let source_code = "={===}";
        let mut scanner = Scanner::new(source_code, &mut error_handler);
        let expected = vec![
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::EOF, 1),
        ];
        let got = scanner.scan_source();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_scanner_as_iterator() {
        let mut error_handler = ErrorHandler::new();
        let source_code = "code";
        let mut scanner = Scanner::new(source_code, &mut error_handler);
        assert_eq!(scanner.prev(), None);
        assert_eq!(scanner.next(), Some('c'));
        assert_eq!(scanner.peek(), Some('o'));
        assert_eq!(scanner.peek_next(), Some('d'));
        assert_eq!(scanner.next(), Some('o'));
        assert_eq!(scanner.next(), Some('d'));
        assert_eq!(scanner.next(), Some('e'));
        assert_eq!(scanner.prev(), Some('e'));
        assert_eq!(scanner.next(), Some('e'));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn test_paren() {
        let mut error_handler = ErrorHandler::new();
        let source_code = "(()";
        let mut scanner = Scanner::new(source_code, &mut error_handler);
        assert_eq!(
            vec![
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::LeftParen, 1),
                Token::new(TokenType::RightParen, 1),
                Token::new(TokenType::EOF, 1)
            ],
            scanner.scan_source()
        );
    }

    #[test]
    fn test_language_scan() {
        let mut error_handler = ErrorHandler::new();
        let source_code = "\
var x = 9;
var y = x + 8.8;
";
        let mut scanner = Scanner::new(source_code, &mut error_handler);
        let tokens = scanner.scan_source();
        assert_eq!(tokens.len(), 13);
    }
}
