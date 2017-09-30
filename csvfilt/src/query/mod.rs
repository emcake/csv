pub struct QueryString(String);

impl QueryString {
    pub fn new(s:String) -> Self { QueryString(s) }
}


enum Op {
    Eq,
    Lt,
    Gt
}

use schema::ColType;
use schema::{OpDouble, OpSingle};

impl ColType {


    fn get_for_op(&self, op : Op) -> OpDouble
    {
        match op {
            Op::Eq => self.eq.0(),
            Op::Gt => self.gt.0(),
            Op::Lt => self.lt.0()
        }
    }

    fn get_for_op_left_baked(&self, op : Op, left:&String) -> OpSingle
    {
        match op {
            Op::Eq => self.eq.1(left),
            Op::Gt => self.gt.1(left),
            Op::Lt => self.lt.1(left)
        }
    }
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
                    Err(From::from(format!("Could not find {} or {} as a column", left, right)))
                }
            (Some((idx_a,col_a)), Some((idx_b,col_b))) =>
                {
                    use std::rc::Rc;
                    if Rc::ptr_eq(&col_a.col_type,&col_b.col_type) {
                        let op = col_a.col_type.get_for_op(op)?;
                        Ok(ColumnOp(Box::new(move |row|{
                            let a = row[idx_a].trim();
                            let b = row[idx_b].trim();
                            op(&a.to_owned(),&b.to_owned())
                        })))
                    }
                    else {
                        Err(From::from(format!("Tried to compare values of {} and {} but the types don't match", left, right)))
                    }
                }
            
            (None, Some((idx,col))) =>
                {
                    let op = col.col_type.get_for_op_left_baked(op, &left)?;
                    Ok(ColumnOp(Box::new(move |row|{
                        let b = row[idx].trim();
                        op(&b.to_owned())
                    })))
                }
            (Some((idx,col)), None) =>
                {
                    let alternate_op = 
                        match op { // we need to reverse comparison operators if baking the right param, as we only know how to bake the left
                            Op::Eq => Op::Eq,
                            Op::Gt => Op::Lt,
                            Op::Lt => Op::Gt
                        };
                    let op_fn = col.col_type.get_for_op_left_baked(alternate_op, &right)?;
                    Ok(ColumnOp(Box::new(move |row|{
                        let a = row[idx].trim();
                        op_fn(&a.to_owned())
                    })))
                }
        }
    }
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
                {left : "price".to_owned(), op : Op::Gt, right : "100".to_owned()}
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