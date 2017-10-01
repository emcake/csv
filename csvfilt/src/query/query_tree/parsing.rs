
use std::iter::Peekable;
use std::slice::Iter;

use std::error::Error;

use query::tokens::Token;
use query::query_tree::{QueryTree, Op};

// S := expr | and | or
// and := expr && S
// or := expr || S
// expr := binop | not | bracketed
// not := !backeted
// bracketed := (S)
// binop := ident op ident
// op := < | > | <= | >= | = | !=

pub fn entry(p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
{
    maybe_and_or(p)
}    


pub fn maybe_and_or(p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
{
    let current = expr(p)?;
    match p.peek() {
        None => Ok(current),
        Some (&tok) => match tok {
            &Token::And => 
                {
                    p.next().unwrap();
                    let other = entry(p)?;
                    Ok(Box::new(
                        QueryTree::And { q1 : current, q2 : other }
                    ))
                }
            &Token::Or =>
                {
                    p.next().unwrap();
                    let other = entry(p)?;
                    Ok(Box::new(
                        QueryTree::Or { q1 : current, q2 : other }
                    ))
                }
            _ =>
                Ok(current)
        }
    }
}

pub fn bracketed(check_open:bool, p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
{
    if check_open
    {
        match p.peek()
        {
            None =>
                return Err(From::from("Expected '(', found <EOL>")),
            Some(&tok) => match tok {
                &Token::OpenBracket =>
                    {
                        p.next().unwrap();
                    }
                t => 
                    return Err(From::from(format!("Expected '(', found {:?}", t)))
                    
            }
        }
    }

    let inner = entry(p)?;

    match p.peek()
    {
        None =>
            return Err(From::from("Expected ')', found <EOL>")),
        Some(&tok) => match tok {
            &Token::CloseBracket =>
                {
                    p.next().unwrap();
                    Ok(inner)
                }
            t => 
                return Err(From::from(format!("Expected ')', found {:?}", t)))
                
        }
    }
}

pub fn expr(p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
{
    match p.peek() {
        None => Err(From::from("Expected expr, got <EOL>")),
        Some (&tok) =>
            match tok {
                &Token::ConstOrIdentifier(ref nm) =>
                    {
                        p.next().unwrap();
                        binop(nm.clone(), p)
                    }
                &Token::Not =>
                    {
                        p.next().unwrap();
                        let inner_q = bracketed(true,p)?;
                        Ok(Box::new(QueryTree::Not { q : inner_q }))
                    }
                &Token::OpenBracket =>
                    {
                        p.next().unwrap();
                        bracketed(false, p)
                    }
                _ =>
                    unimplemented!()
            }
    }
}

fn op (p : &mut Peekable<Iter<Token>>) -> Result<Op, Box<Error>>
{
    match p.next() {
        None => Err(From::from("Expected op, got <EOL>")),
        Some (ref tok) =>  match tok {
            &&Token::Eq => Ok(Op::Eq),
            &&Token::NotEq => Ok(Op::NotEq),
            &&Token::Lt => Ok(Op::Lt),
            &&Token::Gt => Ok(Op::Gt),
            &&Token::GEq => Ok(Op::GEq),
            &&Token::LEq => Ok(Op::LEq),
            x => Err(From::from(format!("Expected op, got {:?}", x)))
        }
    }
}

fn ident (p : &mut Peekable<Iter<Token>>) -> Result<String, Box<Error>>
{
    match p.peek() {
        None => Err(From::from("Expected ident, got <EOL>")),
        Some (&tok) =>  match tok {
            &Token::ConstOrIdentifier(ref id) => 
                {
                    p.next().unwrap();
                    Ok(id.clone())
                }
            x => Err(From::from(format!("Expected ident, got {:?}", *x)))
        }
    }
}

fn binop(left : String, p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
{
    let operation = op(p)?;
    let right = ident(p)?;

    Ok(Box::new(
        QueryTree::Op {left : left, op : operation, right : right}
    ))
}


mod tests {
    use query::tokens::Token;
    use query::query_tree::Op;
    use std::error::Error;

    #[test] 
    fn exhaustive_op_parsing() {
        fn fail_on(tok : Token) -> (Token, Result<Op, Box<Error>>)
        {
            (tok.clone(), Err(From::from(format!("Expected op, got {:?}", tok))))
        }
        let ops : [(Token, Result<Op, Box<Error>>);12] =
                [
                    (Token::Eq , Ok(Op::Eq)),
                    (Token::NotEq , Ok(Op::NotEq)),
                    (Token::Lt , Ok(Op::Lt)),
                    (Token::Gt , Ok(Op::Gt)),
                    (Token::GEq , Ok(Op::GEq)),
                    (Token::LEq , Ok(Op::LEq)),
                    fail_on(Token::OpenBracket),
                    fail_on(Token::CloseBracket),
                    fail_on(Token::ConstOrIdentifier("a".to_owned())),
                    fail_on(Token::Not),
                    fail_on(Token::And),
                    fail_on(Token::Or)
                ];

        use query::query_tree::parsing::op;

        use std::iter::Peekable;

        for &(ref x,ref expected) in ops.iter() {
            let x_copy = x.clone();
            let v = vec!(x_copy);
            let mut stream = v.iter().peekable();
            let actual = op(&mut stream);
            match (expected, actual) {
                (&Err(ref x), Err(ref y)) => {
                    let str_x = format!("{}", x);
                    let str_y = format!("{}", y);
                    assert_eq!(str_x, str_y)
                },
                (&Ok(_), Err(_)) => panic!("doesn't match"),
                (&Err(_), Ok(_)) => panic!("doesn't match"),
                (&Ok(ref x), Ok(ref y)) => assert_eq!(*x, *y),
            }
        }
    }

    use query::tokens::tokenise;

    use query::query_tree::QueryTree;

    #[test]
    fn and_works()
    {
        let test = "foo = true && bar = false".to_owned();
        let tokens = tokenise(&test).unwrap();

        let expected = 
            Box::new(QueryTree::And {
                q1 : Box::new(QueryTree::Op {
                    left : "foo".to_owned(),
                    op : Op::Eq,
                    right : "true".to_owned(),
                }),
                q2 : Box::new(QueryTree::Op {
                    left : "bar".to_owned(),
                    op : Op::Eq,
                    right : "false".to_owned(),
                }),
            });

        use query::query_tree::parsing::entry;
        
        let mut stream = tokens.iter().peekable();
        
        let actual = entry(&mut stream).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn or_works()
    {
        let test = "foo = true || bar = false".to_owned();
        let tokens = tokenise(&test).unwrap();

        let expected = 
            Box::new(QueryTree::Or {
                q1 : Box::new(QueryTree::Op {
                    left : "foo".to_owned(),
                    op : Op::Eq,
                    right : "true".to_owned(),
                }),
                q2 : Box::new(QueryTree::Op {
                    left : "bar".to_owned(),
                    op : Op::Eq,
                    right : "false".to_owned(),
                }),
            });

        use query::query_tree::parsing::entry;
        
        let mut stream = tokens.iter().peekable();
        
        let actual = entry(&mut stream).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn maybe_and_or_is_right_associative()
    {
        let test = "foo = true && bar = false || baz = true".to_owned();
        let tokens = tokenise(&test).unwrap();

        let expected = 
            Box::new(QueryTree::And {
                q1 : Box::new(QueryTree::Op {
                    left : "foo".to_owned(),
                    op : Op::Eq,
                    right : "true".to_owned(),
                }),
                q2 : Box::new(QueryTree::Or {
                    q1 : Box::new(QueryTree::Op {
                        left : "bar".to_owned(),
                        op : Op::Eq,
                        right : "false".to_owned(),
                    }),
                    q2 : Box::new(QueryTree::Op {
                        left : "baz".to_owned(),
                        op : Op::Eq,
                        right : "true".to_owned(),
                    }),
                }),
            });

        use query::query_tree::parsing::entry;
        
        let mut stream = tokens.iter().peekable();
        
        let actual = entry(&mut stream).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn brackets_correctly_override_right_associativity()
    {
        let test = "(foo = true && bar = false) || baz = true".to_owned();
        let tokens = tokenise(&test).unwrap();

        let expected = 
            Box::new(QueryTree::Or {
                q1 : Box::new(QueryTree::And {
                    q1 : Box::new(QueryTree::Op {
                        left : "foo".to_owned(),
                        op : Op::Eq,
                        right : "true".to_owned(),
                    }),
                    q2 : Box::new(QueryTree::Op {
                        left : "bar".to_owned(),
                        op : Op::Eq,
                        right : "false".to_owned(),
                    }),
                }),
                q2 : Box::new(QueryTree::Op {
                    left : "baz".to_owned(),
                    op : Op::Eq,
                    right : "true".to_owned(),
                }),
            });

        use query::query_tree::parsing::entry;
        
        let mut stream = tokens.iter().peekable();
        
        let actual = entry(&mut stream).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn brackets_nest_correctly()
    {
        let test = "(((foo = true)))".to_owned();
        let tokens = tokenise(&test).unwrap();

        let expected = 
            Box::new(QueryTree::Op {
                    left : "foo".to_owned(),
                    op : Op::Eq,
                    right : "true".to_owned(),
                });

        use query::query_tree::parsing::entry;
        
        let mut stream = tokens.iter().peekable();
        
        let actual = entry(&mut stream).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn mismatched_brackets_result_in_error()
    {
        let test = "(((foo = true))".to_owned();
        let tokens = tokenise(&test).unwrap();

        use query::query_tree::parsing::entry;
        
        let mut stream = tokens.iter().peekable();
        
        let actual = entry(&mut stream);

        match actual {
            Err (e) =>
                {
                    let err = format!("{}", e);
                    assert_eq!("Expected ')', found <EOL>", err);
                }
            Ok(_) => panic!("was supposed to fail!")
        }

    }
}