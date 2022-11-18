use petgraph::graph::{DiGraph};
use crate::sheet::Sheet;

type DataFlowGraph = DiGraph<(), ()>;

fn make_dataflow_graph(sheet: &Sheet) -> DataFlowGraph {
    todo!()
}
