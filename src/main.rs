use anyhow::Result;
use calamine::{open_workbook_auto, Reader};
use rollup::dataflow::DataFlowGraph;
use rollup::parse::parse;
use rollup::sheet::{Cell, Formula, Range, Sheet};

fn _main1() {
    let s = "=SUM(1,2)";
    let expected_output = Cell::Formula(Formula::Function(
        "SUM".to_string(),
        vec![Formula::Range(Range {
            start: (0, 0),
            end: (0, 1),
        })],
    ));
    println!("{:?}", parse(s));
    println!("{:?}", expected_output);
}

fn main() -> Result<()> {
    let path = format!("{}/test.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook = open_workbook_auto(path).expect("Cannot open file");

    // You can also get defined names definition (string representation only)
    for name in workbook.defined_names() {
        println!("name: {}, formula: {}", name.0, name.1);
    }

    // Now get all formula!
    let sheets = workbook.sheet_names().to_owned();
    assert!(sheets.len() == 1);
    let sheet = &sheets[0];
    let sheet_data = workbook
        .worksheet_range(&sheet)
        .expect("sheet not found")
        .expect("error while getting data");
    let sheet_formulas = workbook
        .worksheet_formula(&sheet)
        .expect("sheet not found")
        .expect("error while getting formula");

    let bottom_right = sheet_data.end().unwrap();
    let bottom_row = bottom_right.0 as usize;
    let right_col = bottom_right.1 as usize;

    let mut rows: Vec<Vec<Cell>> =
        vec![vec![Cell::Text("".to_string()); right_col + 1]; bottom_row + 1];

    println!("Workbook valid range: {:?}", sheet_data);
    println!("Formula valid range: {:?}", sheet_formulas);

    for row in 0..=bottom_row {
        for col in 0..=right_col {
            let data = sheet_data.get_value((row as u32, col as u32));
            if let Some(data) = data {
                rows[row][col] = Cell::new_text(data.to_string());
            }

            let formula = sheet_formulas.get_value((row as u32, col as u32));
            if let Some(formula) = formula {
                if formula.len() > 0 {
                    println!("formula: {:?}", formula);
                    let formula = format!("={}", formula);
                    let cell = parse(&formula)?;
                    rows[row][col] = cell;
                }
            }
        }
    }

    let sheet = Sheet::from_vec(rows);

    println!("{}", sheet);

    let graph = DataFlowGraph::from_sheet(&sheet);

    graph.viz();

    Ok(())
}
