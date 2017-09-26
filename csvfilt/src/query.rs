pub struct QueryString(String);

impl QueryString {
    pub fn new(s:String) -> Self { QueryString(s) }
}

pub enum Comp {
    Eq,
    Lt,
    Gt
}

enum Query {
    Op {
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

use std::error::{Error};

impl Query {
    pub fn from_qstring(qs:&QueryString) -> Result<Box<Query>, Box<Error>>
    {
        // TODO
        Ok(Box::new(Query::ColumnOp {index : 0, comparator : Comp::Eq, value : String::default()})) // dummy
    }
}

pub struct QueryFn (Box<Fn(&Vec<String>) -> bool>);

use schema::Schema;

impl QueryFn {
    fn from_query(q:Query, s:&Schema) -> Result<QueryFn, Box<Error>> 
    {
        Ok(QueryFn(Box::new(|row|{ true })))
    }

    pub fn matches(&self, row:&Vec<String>) -> bool {
        self.0(row)
    }
}


pub fn parse(q:&QueryString, s:&Schema) -> Result<QueryFn, Box<Error>>
{
    let query = Query::from_qstring(q)?;

    QueryFn::from_query(*query, s)
}