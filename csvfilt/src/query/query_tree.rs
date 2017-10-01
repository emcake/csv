pub enum Op {
    Eq,
    NotEq,
    Lt,
    LEq,
    Gt,
    GEq
}

pub enum QueryTree {
    Op {
        left : String,
        op : Op,
        right : String
    },
    Not {
        q : Box<QueryTree>
    },
    And {
        q1 : Box<QueryTree>,
        q2 : Box<QueryTree>
    },
    Or {
        q1 : Box<QueryTree>,
        q2 : Box<QueryTree>
    }
}

mod parsing {
    use std::iter::Peekable;
    use std::slice::Iter;

    use std::error::Error;

    use query::tokens::Token;
    use query::query_tree::{QueryTree, Op};

    // S := maybe_and_or
    // maybe_and_or := expr | and | or
    // and := S && S
    // or := S || S
    // expr := binop | not | bracketed
    // not := !(expr)
    // bracketed := (expr)
    // binop := ident op ident
    // op := < | > | <= | >= | = | !=

    pub fn entry(p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
    {
        maybe_and_or(p)
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
                            let inner_q = expr(p)?;
                            Ok(Box::new(QueryTree::Not { q : inner_q }))
                        }
                    _ =>
                        unimplemented!()
                }
        }
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
                        let other = expr(p)?;
                        Ok(Box::new(
                            QueryTree::And { q1 : current, q2 : other }
                        ))
                    }
                &Token::Or =>
                    {
                        p.next().unwrap();
                        let other = expr(p)?;
                        Ok(Box::new(
                            QueryTree::Or { q1 : current, q2 : other }
                        ))
                    }
                _ =>
                    Ok(current)
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
}

use std::error::{Error};

use query::tokens::{Token, tokenise};

impl QueryTree {

    fn from_tokens(tokens: Vec<Token>) -> Result<Box<QueryTree>, Box<Error>> {
        let mut peekable = tokens.iter().peekable();
        parsing::entry(&mut peekable)
    }


    pub fn from_qstring(s:&String) -> Result<Box<QueryTree>, Box<Error>>
    {
        // TODO
        //QueryTree::form_op(s, "stock".to_owned(), Op::Eq, "VOD.L".to_owned())?;
        let tokens = tokenise(s)?;

        QueryTree::from_tokens(tokens)
    }
}