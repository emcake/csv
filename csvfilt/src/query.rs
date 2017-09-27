pub struct QueryString(String);

impl QueryString {
    pub fn new(s:String) -> Self { QueryString(s) }
}

struct ColumnOp(Box<Fn(&Vec<String>) -> Result<bool, Box<Error>>>);

impl ColumnOp {
    fn to_fn(self) -> Box<Fn(&Vec<String>) -> Result<bool, Box<Error>>>
    {
        self.0
    }

    fn form_op(schema: &Schema, left:String, op : Op, right:String) -> Result<ColumnOp, Box<Error>>
    {
        match (schema.try_find_col(&left), schema.try_find_col(&right)) {
            (None, None) => // neither are columns, this is probably an error
                {
                    Err(From::from(format!("Could not find {:?} or {:?} as a column", left, right)))
                }
            (Some((idx_a,col_a)), Some((idx_b,col_b))) =>
                {
                    use std::rc::Rc;
                    if Rc::ptr_eq(&col_a.col_type,&col_b.col_type) {
                        let eq = col_a.col_type.eq.0()?;
                        Ok(ColumnOp(Box::new(move |row|{
                            let a = &row[idx_a];
                            let b = &row[idx_b];
                            eq(a,b)
                        })))
                    }
                    else {
                        Err(From::from(format!("Tried to compare values of {:?} and {:?} but the types don't match", left, right)))
                    }
                }
            
            (None, Some((idx,col))) =>
                {
                    let eq = col.col_type.eq.1(&left)?;
                    Ok(ColumnOp(Box::new(move |row|{
                        let b = &row[idx];
                        eq(b)
                    })))
                }
            (Some((idx,col)), None) =>
                {
                    let eq = col.col_type.eq.1(&right)?;
                    Ok(ColumnOp(Box::new(move |row|{
                        let a = &row[idx];
                        eq(a)
                    })))
                }
        }
    }
}


enum Op {
    Eq,
    Lt,
    Gt
}
enum Query {
    Op {
        left : String,
        op : Op,
        right : String
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


    pub fn from_qstring(qs:&QueryString, s:&Schema) -> Result<Box<Query>, Box<Error>>
    {
        // TODO
        //Query::form_op(s, "stock".to_owned(), Op::Eq, "VOD.L".to_owned())?;
        Ok(Box::new(
            Query::Op
                {left : "stock".to_owned(), op : Op::Eq, right : "VOD.L".to_owned()}
                ))
    }
}

pub struct QueryFn (Box<Fn(&Vec<String>) -> Result<bool, Box<Error>>>);

use schema::Schema;

impl QueryFn {
    fn from_query_inner(q:Query, s:&Schema) 
        -> 
            Result<
                Box<
                    Fn(&Vec<String>) -> Result<bool, Box<Error>>
                >, 
                Box<Error>
                >
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
                    Ok(Box::new(move |row|{
                        let a = a(row)?;
                        let b = b(row)?;
                        Ok(a && b)
                        }))
                },
            Query::Or {q1, q2} =>
                {
                    let a = QueryFn::from_query_inner(*q1, s)?;
                    let b = QueryFn::from_query_inner(*q2, s)?;
                    Ok(Box::new(move |row|{
                        let a = a(row)?;
                        let b = b(row)?;
                        Ok(a || b)
                        }))
                },
            Query::Op{ left, op, right } =>
                {
                    Ok(ColumnOp::form_op(s, left, op, right)?.to_fn())
                }
        }
    }

    fn from_query(q:Query, s:&Schema) -> Result<QueryFn, Box<Error>>{
        let inner = QueryFn::from_query_inner(q, s)?;
        Ok(QueryFn(inner))
    } 

    pub fn matches(&self, row:&Vec<String>) -> Result<bool, Box<Error>> {
        self.0(row)
    }
}

pub fn parse(q:&QueryString, s:&Schema) -> Result<QueryFn, Box<Error>>
{
    let query = Query::from_qstring(q, s)?;

    QueryFn::from_query(*query, s)
}