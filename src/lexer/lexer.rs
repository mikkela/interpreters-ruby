use std::rc::Rc;
use crate::token::{Token, Tokenize};

pub struct Lexer<'a, T> where
    T: Token
{
    tokenizer: Rc<dyn Tokenize<T> >,
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char
}

impl<'a, T> Lexer<'a, T>  where
    T: Token
{
    pub fn new (tokenizer: Rc<dyn Tokenize<T>>, input: &'a str) -> Lexer<'a, T> {
        Lexer { tokenizer, input: input, position: 0, read_position: 0, ch: ' ' }
    }
    pub fn next_token(&mut self) -> T {
        self.skip_whitespace();
        let mut token = self.tokenizer.error_token();
        if self.ch == '\0' {
            token = self.tokenizer.end_of_file_token();
        }
        else if self.tokenizer.is_symbol_start_character(self.ch, self.peek_next_char()) {
            token = self.read_token(|r, c, next, s| r.is_symbol_part_character(c, next, s));
        } else if self.tokenizer.is_numeric_start_character(self.ch, self.peek_next_char()) {
            token = self.read_token(|r, c, next, s| r.is_numeric_part_character(c, next, s));
        } else if self.tokenizer.is_identifier_start_character(self.ch, self.peek_next_char()) {
            token = self.read_token(|r, c, next, s| r.is_identifier_part_character(c, next, s));
        }

        self.read_next_char();
        token
    }

    fn read_token(&mut self, predicate: fn( Rc<dyn Tokenize<T>>, char, char, &str) -> bool) -> T {
        let start = self.position;
        let mut ch = self.peek_next_char();
        while predicate(self.tokenizer.clone(), ch, self.peek_next_char(), &self.input[start..self.read_position]) {
            self.read_next_char();
            ch = self.peek_next_char();
        }
        return self.tokenizer.to_token(&self.input[start..self.read_position]);
    }
    fn skip_whitespace(&mut self) {
        while self.tokenizer.is_whitespace_character(self.ch) {
            self.read_next_char();
        }

    }

    fn read_next_char(&mut self) {
        self.ch = self.peek_next_char();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_next_char(&self)-> char {
        if let Some(char_at_index) = self.input.chars().nth(self.read_position) {
            char_at_index
        } else {
            '\0'
        }
    }
}
