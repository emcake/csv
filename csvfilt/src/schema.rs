pub enum ColType {
    TString,
    TInt,
    TFloat,
    TBool
}

pub struct ColItem {
    name : String,
    col_type : ColType
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

    pub fn try_find_col(&self, name:String) -> Option<(usize, &ColItem)> {
        self.0.iter().enumerate().find(|&x|{x.1.name == name})
    }
}