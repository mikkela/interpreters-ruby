use crate::token::{Token, Tokenize};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BasicToken {
    Illegal,
    EOF,
    Name(String),
    Number(isize),
    Plus,
    Minus,
    Asteriks,
    Slash,
    LT,
    GT,
    Eq,
    Print,
    Semicolon,
    LParen,
    RParen,
    Define,
    If,
    While,
    Set,
    Begin,
}

impl Token for BasicToken {
    fn literal(&self) -> String {
        match self {
            BasicToken::Illegal => String::from("ILLEGAL"),
            BasicToken::EOF => String::from("EOF"),
            BasicToken::Name(ident) => String::from(ident),
            BasicToken::Number(value) => String::from(value.to_string().as_str()),
            BasicToken::Plus => String::from("+"),
            BasicToken::Minus => String::from("-"),
            BasicToken::Asteriks => String::from("*"),
            BasicToken::Slash => String::from("/"),
            BasicToken::LT => String::from("<"),
            BasicToken::GT => String::from(">"),
            BasicToken::Eq => String::from("="),
            BasicToken::Print => String::from("PRINT"),
            BasicToken::LParen => String::from("("),
            BasicToken::RParen => String::from(")"),
            BasicToken::Define => String::from("DEFINE"),
            BasicToken::If => String::from("IF"),
            BasicToken::While => String::from("WHILE"),
            BasicToken::Set => String::from("SET"),
            BasicToken::Begin => String::from("BEGIN"),
            BasicToken::Semicolon => String::from(";"),
        }
    }
}

pub struct BasicTokenizer {

}

impl BasicTokenizer {
    fn is_digit(ch: char) -> bool{
        '0' <= ch && ch <= '9'
    }

    fn is_name(ch: char) -> bool{
        !Self::is_digit(ch) &&
            !Self::is_white_space(ch) &&
            ch != '(' && ch != ')' && ch != ';' && ch != '\0'
    }

    fn is_white_space(ch: char) -> bool {
        ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
    }
}
impl Tokenize<BasicToken> for BasicTokenizer {
    fn is_whitespace_character(&self, ch: char) -> bool {
        Self::is_white_space(ch)
    }

    fn is_symbol_start_character(&self, ch: char, next: char) -> bool {
        ch == '('  || ch == ')' || ((ch == '=' || ch == '+' || ch == '-' ||
            ch == '/' || ch == '*' || ch == '<' || ch == '>') && next == ' ')
    }

    fn is_symbol_part_character(&self, _: char, _:char, _: &str) -> bool {
        false
    }

    fn is_numeric_start_character(&self, ch: char, next: char) -> bool {
        BasicTokenizer::is_digit(ch) || ch == '-' && BasicTokenizer::is_digit(next)
    }

    fn is_numeric_part_character(&self, ch: char, _:char, _: &str) -> bool {
        BasicTokenizer::is_digit(ch)
    }

    fn is_identifier_start_character(&self, ch: char, _: char) -> bool {
        BasicTokenizer::is_name(ch)
    }

    fn is_identifier_part_character(&self, ch: char, _: char, _: &str) -> bool {
        BasicTokenizer::is_name(ch)
    }

    fn to_token(&self, s: &str) -> BasicToken {
        match s {
            "=" => BasicToken::Eq,
            "+" => BasicToken::Plus,
            "-" => BasicToken::Minus,
            "/" => BasicToken::Slash,
            "*" => BasicToken::Asteriks,
            "<" => BasicToken::LT,
            ">" => BasicToken::GT,
            ";" => BasicToken::Semicolon,
            "(" => BasicToken::LParen,
            ")" => BasicToken::RParen,
            ";" => BasicToken::Semicolon,
            "print" => BasicToken::Print,
            "define" => BasicToken::Define,
            "if" => BasicToken::If,
            "while" => BasicToken::While,
            "set" => BasicToken::Set,
            "begin" => BasicToken::Begin,
            x => {
                if let Ok(value) = x.parse::<isize>() {
                    BasicToken::Number(value)
                } else {
                    BasicToken::Name(x.to_string())
                }
            }
        }
    }

    fn end_of_file_token(&self) -> BasicToken {
        BasicToken::EOF
    }

    fn error_token(&self) -> BasicToken {
        BasicToken::Illegal
    }
}
