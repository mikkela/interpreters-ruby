use crate::lexer::Lexer;
use crate::basic::token::BasicToken;

pub type BasicLexer<'a> = Lexer<'a, BasicToken>;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use rstest::rstest;
    use crate::basic::lexer::BasicLexer;
    use crate::basic::token::BasicTokenizer;
    use crate::basic::token::BasicToken;
    use crate::token::Tokenize;

    #[rstest]
    #[case("= ", BasicToken::Eq)]
    #[case("+ ", BasicToken::Plus)]
    #[case("- ", BasicToken::Minus)]
    #[case("* ", BasicToken::Asteriks)]
    #[case("/ ", BasicToken::Slash)]
    #[case("< ", BasicToken::LT)]
    #[case("> ", BasicToken::GT)]
    #[case("( ", BasicToken::LParen)]
    #[case(") ", BasicToken::RParen)]
    #[case("print ", BasicToken::Print)]
    #[case("if ", BasicToken::If)]
    #[case("while ", BasicToken::While)]
    #[case("set ", BasicToken::Set)]
    #[case("begin ", BasicToken::Begin)]
    #[case("define ", BasicToken::Define)]
    fn test_symbols_and_keywords(#[case] input: &str, #[case] expected: BasicToken) {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, input);

        assert_eq!(sut.next_token(), expected);
        assert_eq!(sut.next_token(), BasicToken::EOF);
    }

    #[test]
    fn test_positive_numbers() {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, "2345");

        assert_eq!(sut.next_token(), BasicToken::Number(2345));
        assert_eq!(sut.next_token(), BasicToken::EOF)
    }

    #[test]
    fn test_negative_numbers() {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, "-9956");

        assert_eq!(sut.next_token(), BasicToken::Number(-9956));
        assert_eq!(sut.next_token(), BasicToken::EOF)
    }

    #[test]
    fn test_name() {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, "monkey");

        assert_eq!(sut.next_token(), BasicToken::Name(String::from("monkey")));
        assert_eq!(sut.next_token(), BasicToken::EOF)
    }

    #[test]
    fn test_name_2() {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, "<>");

        assert_eq!(sut.next_token(), BasicToken::Name(String::from("<>")));
        assert_eq!(sut.next_token(), BasicToken::EOF)
    }

    #[test]
    fn test_multiple_tokens() {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, "( define not (boolval) ( if boolval 0 1))");

        assert_eq!(sut.next_token(), BasicToken::LParen);
        assert_eq!(sut.next_token(), BasicToken::Define);
        assert_eq!(sut.next_token(), BasicToken::Name(String::from("not")));
        assert_eq!(sut.next_token(), BasicToken::LParen);
        assert_eq!(sut.next_token(), BasicToken::Name(String::from("boolval")));
        assert_eq!(sut.next_token(), BasicToken::RParen);
        assert_eq!(sut.next_token(), BasicToken::LParen);
        assert_eq!(sut.next_token(), BasicToken::If);
        assert_eq!(sut.next_token(), BasicToken::Name(String::from("boolval")));
        assert_eq!(sut.next_token(), BasicToken::Number(0));
        assert_eq!(sut.next_token(), BasicToken::Number(1));
        assert_eq!(sut.next_token(), BasicToken::RParen);
        assert_eq!(sut.next_token(), BasicToken::RParen);
        assert_eq!(sut.next_token(), BasicToken::EOF)
    }

    #[test]
    fn test_right_paranthesis_do_not_need_space() {
        let tokenizer: Rc<dyn Tokenize< BasicToken>> = Rc::new(BasicTokenizer{});
        let mut sut = BasicLexer::new(tokenizer, ")");

        assert_eq!(sut.next_token(), BasicToken::RParen);
        assert_eq!(sut.next_token(), BasicToken::EOF)
    }
}
