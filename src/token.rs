pub mod number;

use maplit::hashmap;
use number::Number;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    None,
    EOF,
    String(String),
    Number(Number),
    Identifier(String),
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    //operators
    Minus,
    Plus,
    Star,
    Slash,
    Equal,

    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,

    // keywords
    Var,
    True,
    False,
    If,
    Else,
    While,
    Class,
    Super,
    For,
    Fun,
    Nil,
}
static LEXEME_TO_TOKENTYPE: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    hashmap! {
        ";" => TokenType::Semicolon,
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "," => TokenType::Comma,
        "." => TokenType::Dot,
        "-" => TokenType::Minus,
        "+" => TokenType::Plus,
        "*" => TokenType::Star,
        "/" => TokenType::Slash,
        "=" => TokenType::Equal,
        "==" => TokenType::EqualEqual,
        "!" => TokenType::Bang,
        "!=" => TokenType::BangEqual,
        "<" => TokenType::Less,
        "<=" => TokenType::LessEqual,
        ">" => TokenType::Greater,
        ">=" => TokenType::GreaterEqual,
        "&" => TokenType::And,
        "|" => TokenType::Or,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "while" => TokenType::While,
        "class" => TokenType::Class,
        "super" => TokenType::Super,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "Nil" => TokenType::Nil,
        "var" => TokenType::Var,
        "" => TokenType::EOF,
    }
});
static TOKENTYPE_TO_STRING: Lazy<HashMap<TokenType, &'static str>> = Lazy::new(|| {
    hashmap! {
        TokenType::EOF => "EOF",
        TokenType::Semicolon=>"SEMICOLON",
        TokenType::LeftParen=>"LEFT_PAREN",
        TokenType::RightParen=>"RIGHT_PAREN",
        TokenType::LeftBrace=>"LEFT_BRACE",
        TokenType::RightBrace=>"RIGHT_BRACE",
        TokenType::Comma    =>"COMMA",
        TokenType::Dot      =>"DOT",
        TokenType::Minus    =>"MINUS",
        TokenType::Plus     =>"PLUS",
        TokenType::Star     =>"STAR",
        TokenType::Slash => "SLASH",
        TokenType::Equal => "EQUAL",
        TokenType::EqualEqual => "EQUAL_EQUAL",
        TokenType::Bang => "BANG",
        TokenType::BangEqual => "BANG_EQUAL",
        TokenType::Less => "LESS",
        TokenType::LessEqual => "LESS_EQUAL",
        TokenType::Greater => "GREATER",
        TokenType::GreaterEqual => "GREATER_EQUAL",
        TokenType::And => "AND",
        TokenType::Or => "OR",
        TokenType::Var => "VAR",
        TokenType::True => "TRUE",
        TokenType::False => "FALSE",
        TokenType::If => "IF",
        TokenType::Else => "ELSE",
        TokenType::While => "WHILE",
        TokenType::Class => "CLASS",
        TokenType::Super => "SUPER",
        TokenType::For => "FOR",
        TokenType::Fun => "FUN",
        TokenType::Nil => "NIL",
        TokenType::Var=> "VAR",

    }
});

impl TokenType {
    pub fn to_str(&self) -> String {
        match self {
            TokenType::String(value) => value.clone(),
            TokenType::Number(num) => num.to_string(),
            TokenType::Identifier(ident) => ident.clone(),
            other_token => {
                let mut ret_lexeme = String::new();
                for (key, value) in LEXEME_TO_TOKENTYPE.iter() {
                    if *value == *other_token {
                        ret_lexeme = key.to_string();
                        break;
                    }
                }
                ret_lexeme
            }
        }
    }
}
impl FromStr for TokenType {
    type Err = String;
    fn from_str(token_string: &str) -> Result<Self, Self::Err> {
        if let Some(token) = LEXEME_TO_TOKENTYPE.get(token_string) {
            Ok(token.clone())
        } else {
            Err(String::from("could not create token type"))
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::String(value) => write!(f, "STRING \"{}\" {}", value, value),
            TokenType::Number(num) => write!(f, "NUMBER {} {}", num, num),
            TokenType::Identifier(ident) => write!(f, "IDENTIFIER {} null", ident),
            other_token => {
                if let Some(token_string) = TOKENTYPE_TO_STRING.get(other_token) {
                    write!(f, "{} {} null", token_string, self.to_str())
                } else {
                    write!(f, "INVALID TOKEN")
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    // pub lexeme: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Token { token_type, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "token_type: {}\nline: {}", self.token_type, self.line)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_display() {
        assert_eq!(
            String::from("NUMBER 90.1 90.1"),
            format!("{}", TokenType::Number(Number::from_str("90.1").unwrap()))
        );
        println!("{}", Number::from_str("90.1").unwrap());
        assert_eq!(String::from("EOF  null"), format!("{}", TokenType::EOF));
        assert_eq!(
            String::from("SEMICOLON ; null"),
            format!("{}", TokenType::Semicolon)
        );
        assert_eq!(
            String::from("IDENTIFIER myident null"),
            format!("{}", TokenType::Identifier(String::from("myident")))
        );
        assert_eq!(
            String::from("STRING \"hello world\" hello world"),
            format!("{}", TokenType::String(String::from("hello world")))
        );
    }
}
