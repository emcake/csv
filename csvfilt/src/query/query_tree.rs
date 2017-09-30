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

use std::error::{Error};

use query::tokens::{Token, tokenise};

impl QueryTree {

    fn from_tokens(tokens: Vec<Token>) -> Result<Box<QueryTree>, Box<Error>> {
        Ok(Box::new(
            QueryTree::Op
                {left : "price".to_owned(), op : Op::Gt, right : "100".to_owned()}
                ))
    }


    pub fn from_qstring(s:&String) -> Result<Box<QueryTree>, Box<Error>>
    {
        // TODO
        //QueryTree::form_op(s, "stock".to_owned(), Op::Eq, "VOD.L".to_owned())?;
        let tokens = tokenise(s)?;

        QueryTree::from_tokens(tokens)
    }
}