use crate::token::{Token, Tokenize};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MonkeyToken {
    Illegal,
    EOF,
    Ident(String),
    Int(isize),
    Assign,
    Plus,
    Minus,
    Bang,
    Asteriks,
    Slash,
    LT,
    GT,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

impl Token for MonkeyToken {
    fn literal(&self) -> String {
        match self {
            MonkeyToken::Illegal => String::from("ILLEGAL"),
            MonkeyToken::EOF => String::from("EOF"),
            MonkeyToken::Ident(ident) => String::from(ident),
            MonkeyToken::Int(value) => String::from(value.to_string().as_str()),
            MonkeyToken::Assign => String::from("="),
            MonkeyToken::Plus => String::from("+"),
            MonkeyToken::Minus => String::from("-"),
            MonkeyToken::Bang => String::from("!"),
            MonkeyToken::Asteriks => String::from("*"),
            MonkeyToken::Slash => String::from("/"),
            MonkeyToken::LT => String::from("<"),
            MonkeyToken::GT => String::from(">"),
            MonkeyToken::Eq => String::from("=="),
            MonkeyToken::NotEq => String::from("!="),
            MonkeyToken::Comma => String::from(","),
            MonkeyToken::Semicolon => String::from(";"),
            MonkeyToken::LParen => String::from("("),
            MonkeyToken::RParen => String::from(")"),
            MonkeyToken::LBrace => String::from("{"),
            MonkeyToken::RBrace => String::from("}"),
            MonkeyToken::Function => String::from("FUNCTION"),
            MonkeyToken::Let => String::from("LET"),
            MonkeyToken::True => String::from("TRUE"),
            MonkeyToken::False => String::from("FALSE"),
            MonkeyToken::If => String::from("IF"),
            MonkeyToken::Else => String::from("ELSE"),
            MonkeyToken::Return => String::from("RETURN"),
        }
    }
}

pub struct MonkeyTokenizer {

}

impl MonkeyTokenizer {
    fn is_digit(ch: char) -> bool{
        '0' <= ch && ch <= '9'
    }

    fn is_letter(ch: char) -> bool{
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }
}
impl Tokenize<MonkeyToken> for MonkeyTokenizer {
    fn is_whitespace_character(&self, ch: char) -> bool {
        ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
    }

    fn is_symbol_start_character(&self, ch: char, _ : char) -> bool {
        ch == '=' || ch == '+' || ch == '-' || ch == '!' ||
            ch == '/' || ch == '*' || ch == '<' || ch == '>' ||
            ch == ';' || ch == '(' || ch == ')' || ch == '{' ||
            ch == '}' || ch == ','
    }

    fn is_symbol_part_character(&self, ch: char, _:char, s: &str) -> bool {
        match s {
            "=" => ch == '=',
            "!" => ch == '=',
            _ => false
        }
    }

    fn is_numeric_start_character(&self, ch: char, _ : char) -> bool {
        MonkeyTokenizer::is_digit(ch)
    }

    fn is_numeric_part_character(&self, ch: char, _:char, _: &str) -> bool {
        MonkeyTokenizer::is_digit(ch)
    }

    fn is_identifier_start_character(&self, ch: char, _ : char) -> bool {
        MonkeyTokenizer::is_letter(ch)
    }

    fn is_identifier_part_character(&self, ch: char, _:char, _: &str) -> bool {
        MonkeyTokenizer::is_letter(ch)
    }

    fn to_token(&self, s: &str) -> MonkeyToken {
        match s {
            "==" => MonkeyToken::Eq,
            "=" => MonkeyToken::Assign,
            "!=" => MonkeyToken::NotEq,
            "!" => MonkeyToken::Bang,
            "+" => MonkeyToken::Plus,
            "-" => MonkeyToken::Minus,
            "/" => MonkeyToken::Slash,
            "*" => MonkeyToken::Asteriks,
            "<" => MonkeyToken::LT,
            ">" => MonkeyToken::GT,
            ";" => MonkeyToken::Semicolon,
            "(" => MonkeyToken::LParen,
            ")" => MonkeyToken::RParen,
            "{" => MonkeyToken::LBrace,
            "}" => MonkeyToken::RBrace,
            "," => MonkeyToken::Comma,
            "fn" => MonkeyToken::Function,
            "let" => MonkeyToken::Let,
            "true" => MonkeyToken::True,
            "false" => MonkeyToken::False,
            "if" => MonkeyToken::If,
            "else" => MonkeyToken::Else,
            "return" => MonkeyToken::Return,
            x => {
                if let Ok(value) = x.parse::<isize>() {
                    MonkeyToken::Int(value)
                } else {
                    MonkeyToken::Ident(x.to_string())
                }
            }
        }
    }

    fn end_of_file_token(&self) -> MonkeyToken {
        MonkeyToken::EOF
    }

    fn error_token(&self) -> MonkeyToken {
        MonkeyToken::Illegal
    }
}
