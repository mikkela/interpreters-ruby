pub trait Token {
    fn literal(&self) -> String;
}

pub trait Tokenize<T>
where T: Token {
    fn is_whitespace_character(&self, ch: char) -> bool;
    fn is_symbol_start_character(&self, ch: char, next: char) -> bool;
    fn is_symbol_part_character(&self, ch: char, next: char, s: &str) -> bool;
    fn is_numeric_start_character(&self, ch: char, next:char) -> bool;
    fn is_numeric_part_character(&self, ch: char, next: char, s: &str) -> bool;
    fn is_identifier_start_character(&self, ch: char, next: char) -> bool;
    fn is_identifier_part_character(&self, ch: char, next: char, s: &str) -> bool;
    fn to_token(&self, s: &str) -> T;

    fn end_of_file_token(&self) -> T;
    fn error_token(&self) -> T;
}
