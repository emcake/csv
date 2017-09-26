pub struct QueryString(String);

pub enum Comp {
    Eq,
    Lt,
    Gt
}

enum Query {
    ColumnOp {
        index : i32,
        comparator : Comp,
        value : String
    },
    And {
        q1 : Box<Query>,
        q2 : Box<Query>
    },
    Or {
        q1 : Box<Query>,
        q2 : Box<Query>
    }
}

struct Schema (());

struct QueryFn (Box<Fn(Vec<String>) -> bool>);

use std::error::{Error};

fn query_to_fn(q:Query, s:&Schema) -> Result<QueryFn, Box<Error>> 
{
    Ok(QueryFn(Box::new(|row|{ true })))
}

fn qstring_to_query(qs:&QueryString) -> Result<Box<Query>, Box<Error>>
{
    Ok(Box::new(Query::ColumnOp {index : 0, comparator : Comp::Eq, value : String::default()})) // dummy
}

pub fn parse(q:&QueryString, s:&Schema) -> Result<QueryFn, Box<Error>>
{
    let query = qstring_to_query(q)?;

    query_to_fn(*query, s)
}