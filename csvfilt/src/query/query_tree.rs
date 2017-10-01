pub enum Op {
    Eq,
    Lt,
    Gt
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

    // S := expr
    // expr := binop | not | bracketed | and | or
    // binop := ident op ident
    // not := !(expr)
    // bracketed := (expr)
    // and := expr && expr
    // or := expr || expr
    // op := < | > | <= | >= | = | !=

    pub fn choose_expr(p : &mut Peekable<Iter<Token>>) -> Result<Box<QueryTree>, Box<Error>>
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
                    _ =>
                        unimplemented!()
                }
        }
    }

    enum IntermediateOp {
        Lt,
        Gt,
        Eq,
        NEq
    }

    fn op (p : &mut Peekable<Iter<Token>>) -> Result<IntermediateOp, Box<Error>>
    {
        match p.next() {
            None => Err(From::from("Expected op, got <EOL>")),
            Some (ref tok) =>  match tok {
                &&Token::Eq => Ok(IntermediateOp::Eq),
                &&Token::NotEq => Ok(IntermediateOp::NEq),
                &&Token::Lt => Ok(IntermediateOp::Lt),
                &&Token::Gt => Ok(IntermediateOp::Gt),
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

        match operation {
            IntermediateOp::Eq =>
                Ok(Box::new(
                    QueryTree::Op {left : left, op : Op::Eq, right : right}
                        )),
            IntermediateOp::NEq =>
                {
                    let eq = Box::new(
                        QueryTree::Op {left : left, op : Op::Eq, right : right}
                            );
                    Ok(Box::new(
                        QueryTree::Not { q : eq }
                    ))
                }
            IntermediateOp::Lt =>
                Ok(Box::new(
                    QueryTree::Op {left : left, op : Op::Lt, right : right}
                        )),
            IntermediateOp::Gt =>
                Ok(Box::new(
                    QueryTree::Op {left : left, op : Op::Gt, right : right}
                        )),
            _ => unimplemented!()
        }
    }
}

use std::error::{Error};

use query::tokens::{Token, tokenise};

impl QueryTree {

    fn from_tokens(tokens: Vec<Token>) -> Result<Box<QueryTree>, Box<Error>> {
        let mut peekable = tokens.iter().peekable();
        parsing::choose_expr(&mut peekable)
    }


    pub fn from_qstring(s:&String) -> Result<Box<QueryTree>, Box<Error>>
    {
        // TODO
        //QueryTree::form_op(s, "stock".to_owned(), Op::Eq, "VOD.L".to_owned())?;
        let tokens = tokenise(s)?;

        QueryTree::from_tokens(tokens)
    }
}