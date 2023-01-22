use rustoku::basic::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str = "2.3.1865.416753982.58.26.1.84.362.9562.8.543.53.14.826.652...483.458.26..826.457.";

    let puzzle = Sudoku::new(str)?;
    let solver = HumanSolver::new();

    if let Some(hint) = solver.next(&puzzle) {
        // Example of what can be done with the hint, i.e. updating a GUI

        let involved = hint.involved_values();
        for pair in involved.iter() {
            // Note that `IndexValuePair::index()` can return 1-D array index, or a 2-D (row, col) tuple (both are zero indexed)
            let _index: usize = pair.index().into();
            let coord: (usize, usize) = pair.index().into();

            // Note that `IndexValuePair::values` returns a collection that implements `FromIterator<Item=T> where T: From<SingleBit>`
            let involved_value = pair.value();

            println!(
                "Involved potential value: {:?}, from coordinate: ({}, {}) ",
                involved_value, coord.0, coord.1
            );
        }

        println!("\n------\n");

        println!("Solving Technique: {}", hint.technique().unwrap());

        // Get a vector of `IndexValuePair`. This is a tuple struct that contains info of the index and value(s)
        let removed_potentials_pairs = hint.removed_potentials();
        for pair in removed_potentials_pairs {
            // Note that `IndexValuePair::index()` can return 1-D array index, or a 2-D (row, col) tuple (both are zero indexed)
            let index: usize = pair.index().into();
            let coord: (usize, usize) = pair.index().into();

            let removed_potentials_vec = pair.value();

            /*
                Note in this example that only one value is in the value collection.  This is not always the case, particularly in solving
                techniques that use multiple candidates such as tuples or pointing / claiming candidates
            */
            println!(
                "Potential values: {:?}, has been removed from index: {}, which is similar to: \n\
            Potential values: {:?}, has been removed from rol/col: ({}, {}). ",
                &removed_potentials_vec, index, removed_potentials_vec, coord.0, coord.1
            );
        }
    }
    Ok(())
}
