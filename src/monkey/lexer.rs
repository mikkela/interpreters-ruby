use crate::lexer::Lexer;
use crate::monkey::token::MonkeyToken;

pub type MonkeyLexer<'a> = Lexer<'a, MonkeyToken>;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use rstest::rstest;
    use crate::monkey::lexer::MonkeyLexer;
    use crate::monkey::token::MonkeyTokenizer;
    use crate::monkey::token::MonkeyToken;
    use crate::token::Tokenize;

    #[rstest]
    #[case("=", MonkeyToken::Assign)]
    #[case("==", MonkeyToken::Eq)]
    #[case("!", MonkeyToken::Bang)]
    #[case("!=", MonkeyToken::NotEq)]
    #[case("+", MonkeyToken::Plus)]
    #[case("-", MonkeyToken::Minus)]
    #[case("*", MonkeyToken::Asteriks)]
    #[case("/", MonkeyToken::Slash)]
    #[case("<", MonkeyToken::LT)]
    #[case(">", MonkeyToken::GT)]
    #[case(";", MonkeyToken::Semicolon)]
    #[case("(", MonkeyToken::LParen)]
    #[case(")", MonkeyToken::RParen)]
    #[case("{", MonkeyToken::LBrace)]
    #[case("}", MonkeyToken::RBrace)]
    #[case(",", MonkeyToken::Comma)]
    #[case("fn", MonkeyToken::Function)]
    #[case("let", MonkeyToken::Let)]
    #[case("true", MonkeyToken::True)]
    #[case("false", MonkeyToken::False)]
    #[case("if", MonkeyToken::If)]
    #[case("else", MonkeyToken::Else)]
    #[case("return", MonkeyToken::Return)]
    fn test_symbols_and_keywords(#[case] input: &str, #[case] expected: MonkeyToken) {
        let tokenizer: Rc<dyn Tokenize< MonkeyToken>> = Rc::new(MonkeyTokenizer{});
        let mut sut = MonkeyLexer::new(tokenizer, input);

        assert_eq!(sut.next_token(), expected);
        assert_eq!(sut.next_token(), MonkeyToken::EOF);
    }

    #[test]
    fn test_ints() {
        let tokenizer: Rc<dyn Tokenize< MonkeyToken>> = Rc::new(MonkeyTokenizer{});
        let mut sut = MonkeyLexer::new(tokenizer, "2345");

        assert_eq!(sut.next_token(), MonkeyToken::Int(2345));
        assert_eq!(sut.next_token(), MonkeyToken::EOF)
    }

    #[test]
    fn test_idents() {
        let tokenizer: Rc<dyn Tokenize< MonkeyToken>> = Rc::new(MonkeyTokenizer{});
        let mut sut = MonkeyLexer::new(tokenizer, "monkey");

        assert_eq!(sut.next_token(), MonkeyToken::Ident(String::from("monkey")));
        assert_eq!(sut.next_token(), MonkeyToken::EOF)
    }

    #[test]
    fn test_multiple_tokens() {
        let tokenizer: Rc<dyn Tokenize< MonkeyToken>> = Rc::new(MonkeyTokenizer{});
        let mut sut = MonkeyLexer::new(tokenizer, "let monkey = true");

        assert_eq!(sut.next_token(), MonkeyToken::Let);
        assert_eq!(sut.next_token(), MonkeyToken::Ident(String::from("monkey")));
        assert_eq!(sut.next_token(), MonkeyToken::Assign);
        assert_eq!(sut.next_token(), MonkeyToken::True);
        assert_eq!(sut.next_token(), MonkeyToken::EOF)
    }

    #[test]
    fn test_edge_cases() {
        let tokenizer: Rc<dyn Tokenize< MonkeyToken>> = Rc::new(MonkeyTokenizer{});
        let mut sut = MonkeyLexer::new(tokenizer, "monkey123 <= ! = 98;nam");

        assert_eq!(sut.next_token(), MonkeyToken::Ident(String::from("monkey")));
        assert_eq!(sut.next_token(), MonkeyToken::Int(123));
        assert_eq!(sut.next_token(), MonkeyToken::LT);
        assert_eq!(sut.next_token(), MonkeyToken::Assign);
        assert_eq!(sut.next_token(), MonkeyToken::Bang);
        assert_eq!(sut.next_token(), MonkeyToken::Assign);
        assert_eq!(sut.next_token(), MonkeyToken::Int(98));
        assert_eq!(sut.next_token(), MonkeyToken::Semicolon);
        assert_eq!(sut.next_token(), MonkeyToken::Ident(String::from("nam")));
        assert_eq!(sut.next_token(), MonkeyToken::EOF)
    }
}
