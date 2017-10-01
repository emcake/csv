
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
