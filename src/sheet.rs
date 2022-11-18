#[derive(Debug, PartialEq)]
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}

#[derive(Debug, PartialEq)]
pub enum Cell {
    Text(String),
    Formula(Formula),
}

#[derive(Hash, Debug, PartialEq)]
pub struct Range {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Range {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Range { start, end }
    }
}

#[derive(Debug, PartialEq)]
pub enum Formula {
    Number(f64),
    Text(String),
    Range(Range),
    // name, args
    Function(String, Vec<Formula>),
}
