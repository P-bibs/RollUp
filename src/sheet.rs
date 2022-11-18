
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}

pub enum Cell {
    Text(String),
    Formula(Formula),
}

pub struct Range {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub enum Formula {
    Number(f64),
    Text(String),
    Range(Range),
    // name, args
    Function(String, Vec<Formula>),
}

