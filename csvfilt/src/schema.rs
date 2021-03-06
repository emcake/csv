use std::error::Error;

use std::str::FromStr;

pub trait SupportedColType : FromStr
{
    fn str_type() -> String;

    fn parse_err(value:&String) -> Box<Error> {
        From::from(
            format!(
                "Could not make a {} from '{}'",
                Self::str_type(),
                value
            )
        )
    }
}

pub type OpDouble = Result<Box<Fn(&String, &String) -> Result<bool, Box<Error>>>, Box<Error>>;
pub type OpSingle = Result<Box<Fn(&String) -> Result<bool, Box<Error>>>, Box<Error>>;

pub trait EqMaker : SupportedColType {
    fn make_eq() -> OpDouble
    {
        Err(From::from(format!("{} does not support equality comparison", Self::str_type())))
    }

    fn make_eq_left_const(left:&String) -> OpSingle
    {
        Err(From::from(format!("{} does not support equality comparison", Self::str_type())))        
    }

    fn make_neq() -> OpDouble
    {
        Err(From::from(format!("{} does not support equality comparison", Self::str_type())))
    }

    fn make_neq_left_const(left:&String) -> OpSingle
    {
        Err(From::from(format!("{} does not support equality comparison", Self::str_type())))        
    }
}

impl<T : SupportedColType + PartialEq + FromStr + 'static> EqMaker for T
{
    fn make_eq() -> OpDouble
    {
        Ok(
            Box::new(
                |a,b|{ 
                    let a = a.parse::<T>().map_err(|_|{Self::parse_err(a)})?;
                    let b = b.parse::<T>().map_err(|_|{Self::parse_err(b)})?;
                    Ok(a == b)
                 }
            ))
    }
    fn make_eq_left_const(left:&String) -> OpSingle
    {
        let left_c = left.parse::<T>().map_err(|_|{Self::parse_err(left)})?;
        Ok(
            Box::new(
                move |x|{ 
                    let x = x.parse::<T>().map_err(|_|{Self::parse_err(x)})?;
                    Ok(left_c == x)
                 }
            ))        
    }
    fn make_neq() -> OpDouble
    {
        Ok(
            Box::new(
                |a,b|{ 
                    let a = a.parse::<T>().map_err(|_|{Self::parse_err(a)})?;
                    let b = b.parse::<T>().map_err(|_|{Self::parse_err(b)})?;
                    Ok(a != b)
                 }
            ))
    }
    fn make_neq_left_const(left:&String) -> OpSingle
    {
        let left_c = left.parse::<T>().map_err(|_|{Self::parse_err(left)})?;
        Ok(
            Box::new(
                move |x|{ 
                    let x = x.parse::<T>().map_err(|_|{Self::parse_err(x)})?;
                    Ok(left_c != x)
                 }
            ))        
    }
}

pub trait CompMaker : SupportedColType {
    fn make_lt() -> OpDouble
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))
    }
    fn make_lt_left_const(left:&String) -> OpSingle
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))        
    }
    fn make_leq() -> OpDouble
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))
    }
    fn make_leq_left_const(left:&String) -> OpSingle
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))        
    }
    fn make_gt() -> OpDouble
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))
    }
    fn make_gt_left_const(left:&String) -> OpSingle
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))        
    }
    fn make_geq() -> OpDouble
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))
    }
    fn make_geq_left_const(left:&String) -> OpSingle
    {
        Err(From::from(format!("{} does not support order comparison", Self::str_type())))        
    }
}

impl<T : SupportedColType + PartialOrd + FromStr + 'static> CompMaker for T
{
    fn make_lt() -> OpDouble
    {
        Ok(
            Box::new(
                |a,b|{ 
                    let a = a.parse::<T>().map_err(|_|{Self::parse_err(a)})?;
                    let b = b.parse::<T>().map_err(|_|{Self::parse_err(b)})?;                
                    Ok(a < b)
                 }
            ))
    }
    fn make_lt_left_const(left:&String) -> OpSingle
    {
        let left_c = left.parse::<T>().map_err(|_|{Self::parse_err(left)})?;
        Ok(
            Box::new(
                move |x|{ 
                    let x = x.parse::<T>().map_err(|_|{Self::parse_err(x)})?;
                    Ok(left_c < x)
                 }
            ))        
    }
    fn make_leq() -> OpDouble
    {
        Ok(
            Box::new(
                |a,b|{ 
                    let a = a.parse::<T>().map_err(|_|{Self::parse_err(a)})?;
                    let b = b.parse::<T>().map_err(|_|{Self::parse_err(b)})?;                
                    Ok(a <= b)
                 }
            ))
    }
    fn make_leq_left_const(left:&String) -> OpSingle
    {
        let left_c = left.parse::<T>().map_err(|_|{Self::parse_err(left)})?;
        Ok(
            Box::new(
                move |x|{ 
                    let x = x.parse::<T>().map_err(|_|{Self::parse_err(x)})?;
                    Ok(left_c <= x)
                 }
            ))        
    }

    fn make_gt() -> OpDouble
    {
        Ok(
            Box::new(
                |a,b|{ 
                    let a = a.parse::<T>().map_err(|_|{Self::parse_err(a)})?;
                    let b = b.parse::<T>().map_err(|_|{Self::parse_err(b)})?;           
                    Ok(a > b)
                 }
            ))
    }
    fn make_gt_left_const(left:&String) -> OpSingle
    {
        let left_c = left.parse::<T>().map_err(|_|{Self::parse_err(left)})?;
        Ok(
            Box::new(
                move |x|{ 
                    let x = x.parse::<T>().map_err(|_|{Self::parse_err(x)})?;
                    Ok(left_c > x)
                 }
            ))        
    }
    fn make_geq() -> OpDouble
    {
        Ok(
            Box::new(
                |a,b|{ 
                    let a = a.parse::<T>().map_err(|_|{Self::parse_err(a)})?;
                    let b = b.parse::<T>().map_err(|_|{Self::parse_err(b)})?;           
                    Ok(a >= b)
                 }
            ))
    }
    fn make_geq_left_const(left:&String) -> OpSingle
    {
        let left_c = left.parse::<T>().map_err(|_|{Self::parse_err(left)})?;
        Ok(
            Box::new(
                move |x|{ 
                    let x = x.parse::<T>().map_err(|_|{Self::parse_err(x)})?;
                    Ok(left_c >= x)
                 }
            ))        
    }
}

type OpMakerDouble = Box<Fn() -> OpDouble>;
type OpMakerSingle = Box<Fn(&String) -> OpSingle>;

type MakerPair = (OpMakerDouble, OpMakerSingle);

pub struct ColType {
    pub name : String,
    pub eq : MakerPair,
    pub neq : MakerPair,
    pub lt : MakerPair,
    pub leq : MakerPair,
    pub gt : MakerPair,
    pub geq : MakerPair
}

impl ColType {
    fn make<T : EqMaker + CompMaker + SupportedColType + 'static>() -> Self {
        ColType { 
            name : <T as SupportedColType>::str_type(), 
            eq : 
                (Box::new(<T as EqMaker>::make_eq), Box::new(<T as EqMaker>::make_eq_left_const)), 
            neq : 
                (Box::new(<T as EqMaker>::make_neq), Box::new(<T as EqMaker>::make_neq_left_const)), 
            lt : 
                (Box::new(<T as CompMaker>::make_lt), Box::new(<T as CompMaker>::make_lt_left_const)), 
            leq : 
                (Box::new(<T as CompMaker>::make_leq), Box::new(<T as CompMaker>::make_leq_left_const)), 
            gt : 
                (Box::new(<T as CompMaker>::make_gt), Box::new(<T as CompMaker>::make_gt_left_const)), 
            geq : 
                (Box::new(<T as CompMaker>::make_geq), Box::new(<T as CompMaker>::make_geq_left_const)) 
            }
    }
}

impl SupportedColType for String {
    fn str_type() -> String {
        "string".to_owned()
    }
}

impl SupportedColType for f32 {
    fn str_type() -> String {
        "float".to_owned()
    }
}

impl SupportedColType for bool {
    fn str_type() -> String {
        "bool".to_owned()
    }
}

impl SupportedColType for i32 {
    fn str_type() -> String {
        "int".to_owned()
    }
}

use std::rc::Rc;

struct ColTypes {
    pickers : Vec<Rc<ColType>>
}

impl ColTypes {
    fn make() -> Self {
        ColTypes {
            pickers: vec!(
                Rc::new(ColType::make::<String>()),
                Rc::new(ColType::make::<i32>()),
                Rc::new(ColType::make::<f32>()),
                Rc::new(ColType::make::<bool>())
            )
        }
    }

    fn find(&self, name:&String) -> Result<Rc<ColType>, Box<Error>>
    {
        let o = self.pickers.iter().find(|p|{ p.name == *name }).map(|p|{ p.clone() });
        o.ok_or(From::from(format!("unable to find type matching '{}'", *name)))
    }
}

pub struct ColItem {
    name : String,
    pub col_type : Rc<ColType>
}

impl ColItem {
    fn parse(s:&String, types : &ColTypes) -> Result<Self, Box<Error>> 
    {
        use regex::Regex;
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(?P<colname>\w+)\[(?P<coltype>\w+)\]$").unwrap();
        }
        use regex::Captures;
        let caps = REGEX.captures_iter(s).collect::<Vec<Captures>>();
        if caps.len() == 1 {
            let colname = caps[0]["colname"].to_owned();
            let coltype = caps[0]["coltype"].to_owned();
            Ok(ColItem { name : colname, col_type : types.find(&coltype)? })
        }
        else {
            Err(From::from(format!("Failed to parse {} to a name/type pair", s)))
        }
    }
}

pub struct Schema (Vec<ColItem>);

impl Schema {
    pub fn from_header(header:&Vec<String>) -> Result<Schema, Box<Error>> {
        let types = ColTypes::make();

        let items : Result<Vec<_>,Box<Error>> = 
            header.iter().map(|c|{ ColItem::parse(c, &types) }).collect();

        Ok(Schema(items?))
    }

    pub fn try_find_col(&self, name:&String) -> Option<(usize, &ColItem)> {
        self.0.iter().enumerate().find(|&x|{x.1.name == *name})
    }
}