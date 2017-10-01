
#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    ConstOrIdentifier(String),
    Eq,
    Not,
    NotEq,
    Lt,
    LEq,
    Gt,
    GEq,
    And,
    Or
}

use std::str::Chars;
use std::iter::Peekable;

fn consume_while<F>(it: &mut Peekable<Chars>, x: F) -> Vec<char>
    where F : Fn(char) -> bool {
 
    let mut v: Vec<char> = vec![];
 
    while let Some(&ch) = it.peek() {
        if x(ch) {
            it.next().unwrap();
            v.push(ch);
        } else {
            break;
        }
    }
    v
}

use std::error::Error;

pub fn tokenise(s : &String) -> Result<Vec<Token>, Box<Error>> {
    let mut char_stream = s.chars().peekable();

    let mut tokens = Vec::new();

    loop {
        match char_stream.peek() {
            None => break,
            Some (&c) =>
                match c {
                    '(' => {
                        char_stream.next().unwrap();
                        tokens.push(Token::OpenBracket);
                    }
                    ')' => {
                        char_stream.next().unwrap();
                        tokens.push(Token::CloseBracket);
                    }
                    '<' => {
                        char_stream.next().unwrap();
                        match char_stream.peek() {
                            Some(&'=') => {
                                char_stream.next().unwrap();
                                tokens.push(Token::LEq)
                            }
                            _ =>
                                tokens.push(Token::Lt)              
                        }
                    }
                    '=' => {
                        char_stream.next().unwrap();
                        tokens.push(Token::Eq)
                    }
                    '>' => {
                        char_stream.next().unwrap();
                        match char_stream.peek() {
                            Some(&'=') => {
                                char_stream.next().unwrap();
                                tokens.push(Token::GEq)
                            }
                            _ =>
                                tokens.push(Token::Gt)              
                        }
                    }
                    '!' => {
                        char_stream.next().unwrap();
                        match char_stream.peek() {
                            Some(&'=') => {
                                char_stream.next().unwrap();
                                tokens.push(Token::NotEq)
                            }
                            _ =>
                                tokens.push(Token::Not)              
                        }
                    }
                    '&' => {
                        char_stream.next().unwrap();
                        match char_stream.peek() {
                            Some(&'&') => {
                                char_stream.next().unwrap();
                                tokens.push(Token::And)
                            }
                            Some(&other) => {
                                return Err(From::from(format!("expected '&&' - found '&{}'", other)))
                            }
                            None => {
                                return Err(From::from("expected '&&' - found '&<EOF>'"))                                
                            }
                        }
                    }
                    '|' => {
                        char_stream.next().unwrap();
                        match char_stream.peek() {
                            Some(&'|') => {
                                char_stream.next().unwrap();
                                tokens.push(Token::Or)
                            }
                            Some(&other) => {
                                return Err(From::from(format!("expected '||' - found '|{}'", other)))
                            }
                            None => {
                                return Err(From::from("expected '||' - found '|<EOF>'"))                                
                            }
                        }
                    }
                    x => {
                        fn is_allowed_in_identifier(c:char) -> bool
                        {
                            c.is_alphanumeric() || c == '.' // TODO think here about what is valid
                        }

                        if x.is_whitespace() {
                            // consume, don't use
                            char_stream.next().unwrap();                        
                        }
                        else if is_allowed_in_identifier(x) { 
                            use std::iter::FromIterator;
                            let s = String::from_iter(consume_while(&mut char_stream, is_allowed_in_identifier));
                            tokens.push(Token::ConstOrIdentifier(s));
                        }
                        else {
                            return Err(From::from(format!("Unexpected token {}", x)))
                        }
                    }
                }
        }
    }
    Ok(tokens)
}

mod tests {

    use query::tokens::{tokenise, Token};

    lazy_static! {
        static ref ALL_TOKENS : Vec<Token> = vec!(
            Token::OpenBracket,
            Token::CloseBracket,
            Token::Eq,
            Token::Lt,
            Token::LEq,
            Token::Gt,
            Token::GEq,
            Token::NotEq,
            Token::Not,
            Token::And,
            Token::Or,
            Token::ConstOrIdentifier("abc".to_owned())
        );
    }

    #[test]
    fn tokenise_recognises_all_chars() {
        let s = "()=<<=>>=!=!&&||abc".to_owned();

        let actual = tokenise(&s).unwrap();

        for (e,a) in ALL_TOKENS.iter().zip(actual.iter())
        {
            assert_eq!(e, a);
            
        }
    }

    #[test]
    fn whitespace_doesnt_matter() {
        let s = "( ) = < <= > >= != ! && || abc".to_owned();

        let actual = tokenise(&s).unwrap();

        for (e,a) in ALL_TOKENS.iter().zip(actual.iter())
        {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn whitespace_separates_constants() {
        let s = "abc def".to_owned();

        let expected = 
            vec!(
                Token::ConstOrIdentifier("abc".to_owned()),
                Token::ConstOrIdentifier("def".to_owned())
            );

        let actual = tokenise(&s).unwrap();

        for (e,a) in expected.iter().zip(actual.iter())
        {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn test_not_vs_noteq() {
        let s = "!a!b!!=c!=!d".to_owned();
        let expected = 
            vec!(
                Token::Not,
                Token::ConstOrIdentifier("a".to_owned()),
                Token::Not,
                Token::ConstOrIdentifier("b".to_owned()),
                Token::Not,
                Token::NotEq,               
                Token::ConstOrIdentifier("c".to_owned()),
                Token::NotEq,
                Token::Not,               
                Token::ConstOrIdentifier("d".to_owned())
            );
        let actual = tokenise(&s).unwrap();

        for (e,a) in expected.iter().zip(actual.iter())
        {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn incomplete_and_causes_error() {
        let s1 = "abc & def".to_owned();

        match tokenise(&s1) {
            Ok(_) =>
                panic!("Expected failure, got success"),
            _ => ()
        }

        let s2 = "&".to_owned();

        match tokenise(&s2) {
            Ok(_) =>
                panic!("Expected failure, got success"),
            _ => ()
        }
    }

    #[test]
    fn incomplete_or_causes_error() {
        let s1 = "abc | def".to_owned();

        match tokenise(&s1) {
            Ok(_) =>
                panic!("Expected failure, got success"),
            _ => ()
            
        }

        let s2 = "|".to_owned();

        match tokenise(&s2) {
            Ok(_) =>
                panic!("Expected failure, got success"),
            _ => ()
        }
    }
}