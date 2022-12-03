use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}

impl Sheet {
    pub fn new() -> Self {
        Sheet { data: vec![] }
    }
    pub fn from_vec(data: Vec<Vec<Cell>>) -> Self {
        Sheet { data }
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.data.iter()
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct CellIndex {
    pub col: usize,
    pub row: usize,
}
impl CellIndex {
    pub fn new(col: usize, row: usize) -> Self {
        CellIndex { col, row }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Text(String),
    Formula(Formula),
}

impl Cell {
    pub fn new_text(text: String) -> Self {
        Cell::Text(text)
    }
    // pub fn new_formula(formula: &Cell) -> Self {

    // }
    pub fn get_dependencies(&self) -> Vec<CellIndex> {
        match self {
            Cell::Text(_) => vec![],
            Cell::Formula(formula) => formula.get_dependencies(),
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub struct Range {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Range {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Range { start, end }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Formula {
    Number(f64),
    Text(String),
    Range(Range),
    // name, args
    Function(String, Vec<Formula>),
}

impl Formula {
    pub fn get_dependencies(&self) -> Vec<CellIndex> {
        match self {
            Formula::Number(_) => vec![],
            Formula::Text(_) => vec![],
            Formula::Range(range) => {
                let mut dependencies = vec![];
                for row in range.start.1..=range.end.1 {
                    for col in range.start.0..=range.end.0 {
                        dependencies.push(CellIndex::new(col, row));
                    }
                }
                println!("dependencies: {:?}", dependencies);
                dependencies
            }
            Formula::Function(_, args) => {
                let mut dependencies = vec![];
                for arg in args {
                    dependencies.extend(arg.get_dependencies());
                }
                dependencies
            }
        }
    }
}
