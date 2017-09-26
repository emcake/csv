pub struct QueryString(String);

impl QueryString {
    pub fn new(s:String) -> Self { QueryString(s) }
}

struct ColumnOp(Box<Fn(&Vec<String>) -> bool>);

impl ColumnOp {
    fn to_fn(self) -> Box<Fn(&Vec<String>) -> bool>
    {
        self.0
    }
}

enum Query {
    Op(ColumnOp),
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
        Ok(Box::new(Query::Op(ColumnOp(Box::new(|row|{row[0] == "VOD.L"}))))) // dummy
    }
}

pub struct QueryFn (Box<Fn(&Vec<String>) -> bool>);

use schema::Schema;

impl QueryFn {
    fn from_query_inner(q:Query, s:&Schema) -> Result<Box<Fn(&Vec<String>) -> bool>, Box<Error>> 
    {
        /*
        fn const_const_eq(a:String, b:String) -> Box<Fn(&Vec<String>) -> bool> {
            let eq = a == b;
            Box::new(move |row|{eq})
        };
        fn const_look_eq(con:String, look:usize) -> Box<Fn(&Vec<String>) -> bool> {
            Box::new(move |row|{row[look] == con})
        };
        fn look_look_eq(look1:usize, look2:usize) -> Box<Fn(&Vec<String>) -> bool> {
            Box::new(move |row|{row[look1] == row[look2]})
        };
        */

        match q {
            Query::And {q1, q2} =>
                {
                    let a = QueryFn::from_query_inner(*q1, s)?;
                    let b = QueryFn::from_query_inner(*q2, s)?;
                    Ok(Box::new(move |row|{a(row) && b(row)}))
                },
            Query::Or {q1, q2} =>
                {
                    let a = QueryFn::from_query_inner(*q1, s)?;
                    let b = QueryFn::from_query_inner(*q2, s)?;
                    Ok(Box::new(move |row|{a(row) || b(row)}))
                },
            Query::Op(col_op) =>
                {
                    Ok(col_op.to_fn())
                }
        }
    }

    fn from_query(q:Query, s:&Schema) -> Result<QueryFn, Box<Error>>{
        let inner = QueryFn::from_query_inner(q, s)?;
        Ok(QueryFn(inner))
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