use crate::sheet::{CellIndex, Sheet};
use petgraph::{
    dot::{Config, Dot},
    graphmap::DiGraphMap,
};
use std::fs::File;
use std::io::Write;

pub struct DataFlowGraph(DiGraphMap<CellIndex, ()>);

impl DataFlowGraph {
    pub fn new() -> Self {
        DataFlowGraph(DiGraphMap::new())
    }
    pub fn from_sheet(sheet: &Sheet) -> Self {
        let mut graph = DataFlowGraph::new();
        for (i, row) in sheet.rows().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                for dep in cell.get_dependencies() {
                    graph.0.add_edge(dep, CellIndex::new(j, i), ());
                }
            }
        }
        graph
    }

    pub fn viz(&self) {
        let mut w = File::create("./test.dot").unwrap();
        println!("{:?}", Dot::with_config(&self.0, &[Config::EdgeNoLabel]));
        let s = format!("{:?}", Dot::with_config(&self.0, &[Config::EdgeNoLabel]));
        w.write_all(s.as_bytes()).unwrap();
    }
}
