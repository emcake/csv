#[derive(PartialEq)]
pub enum ColType {
    TString,
    TInt,
    TFloat,
    TBool
}

use std::error::Error;

impl ColType {
    pub fn from_name(s:String) -> Result<ColType, Box<Error>>
    {
        if s == "string" {
            Ok(ColType::TString)
        }
        else if s == "float" {
            Ok(ColType::TFloat)
        }
        else if s == "int" {
            Ok(ColType::TInt)
        }
        else if s == "bool" {
            Ok(ColType::TBool)
        }
        else {
            Err(From::from(format!("failed to parse {:?} to a col. type", s)))
        }
    }

    pub fn make_eq(&self) -> Result<Box<Fn(&String, &String) -> Result<bool, Box<Error>>>, Box<Error>>
    {
        match self {
            &ColType::TString =>
                {
                    Ok(Box::new(|a,b|{Ok(a == b)}))
                }
            &ColType::TFloat =>
                {
                    fn parse_float(s:&String) -> Result<f32, Box<Error>> {
                        match s.parse() 
                        {
                            Ok (f) => Ok(f),
                            Err (e) => Err(Box::new(e))
                        }
                    }
                    Ok(Box::new(|a,b|{
                        let fa = parse_float(a)?;
                        let fb = parse_float(b)?;
                        Ok(fa == fb)
                    }))
                }
            &ColType::TInt =>
                {
                    fn parse_int(s:&String) -> Result<i32, Box<Error>> {
                        match s.parse() 
                        {
                            Ok (f) => Ok(f),
                            Err (e) => Err(Box::new(e))
                        }
                    }
                    Ok(Box::new(|a,b|{
                        let ia = parse_int(a)?;
                        let ib = parse_int(b)?;
                        Ok(ia == ib)
                    }))
                }
            &ColType::TBool =>
                {
                    fn parse_bool(s:&String) -> Result<bool, Box<Error>> {
                        match s.parse() 
                        {
                            Ok (f) => Ok(f),
                            Err (e) => Err(Box::new(e))
                        }
                    }
                    Ok(Box::new(|a,b|{
                        let ba = parse_bool(a)?;
                        let bb = parse_bool(b)?;
                        Ok(ba == bb)
                    }))
                }
        }
    }
}

pub struct ColItem {
    name : String,
    pub col_type : ColType
}

pub struct Schema (Vec<ColItem>);

impl Schema {
    pub fn from_header(header:&Vec<String>) -> Schema {
        Schema(vec![
            ColItem { name : "stock".to_owned(), col_type : ColType::TString },
            ColItem { name : "price".to_owned(), col_type : ColType::TFloat },
            ColItem { name : "size".to_owned(), col_type : ColType::TInt },
            ColItem { name : "executed".to_owned(), col_type : ColType::TBool }
        ])
    }

    pub fn try_find_col(&self, name:&String) -> Option<(usize, &ColItem)> {
        self.0.iter().enumerate().find(|&x|{x.1.name == *name})
    }
}