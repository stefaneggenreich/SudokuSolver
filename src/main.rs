use std::{borrow::BorrowMut, cell, collections::HashMap, fmt, io::empty, vec};

struct Sudoku {
    is_valid: bool,
    field: Vec<i32>,
}

impl Sudoku {}
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field: \n")?;
        let mut line = String::new();

        for (id, element) in self.field.iter().enumerate() {
            // let element: usize = id;
            if id == self.field.len() - 1 {
                line.push_str(format!("| {element} ").as_str());
            }

            if id % 27 == 0 && id > 0 || id == self.field.len() - 1 {
                writeln!(f, "{line}")?;
                line.clear();
                line.push_str(format!("---------------------------------------").as_str());
                writeln!(f, "{line}")?;
                line.clear();
            } else if id % 9 == 0 && id > 0 {
                writeln!(f, "{line}")?;
                line.clear();
            }
            // print!("{id}: ");
            line.push_str(format!("| {element} ").as_str());

            // if id % 3 == 0 && id > 0 {
            //     line.push_str(format!("|").as_str());
            // }
        }
        Ok(())
    }
}

impl Default for Sudoku {
    fn default() -> Sudoku {
        Sudoku {
            is_valid: true,
            field: vec![0; 3 * 3 * 9],
        }
    }
}

fn solver(sudoku: Box<Sudoku>) -> bool {
    // Steps:
    // 1. Find index of all empty cells
    // let mut field = field.clone();
    let mut empty_cells: HashMap<usize, &i32> = HashMap::new();
    sudoku.field.iter().enumerate().for_each(|(id, cell)| {
        if *cell == 0 {
            empty_cells.insert(id, cell);
        }
    });

    // 2. Insert value ( beginning from 1 ) into first empty cell
    let mut new_field = sudoku.field.clone();

    for cell_idx in sudoku.field.iter() {
        if *cell_idx == 0 {
            for number in 1..9 {
                // 3. Evaluate if conditions are violated?
                new_field[*cell_idx as usize] = number;
                if iter_lines(&new_field) {
                    // sudoku.field = new_field;
                        solver(sudoku);
                }
            }
        }
    }
     true
}

// fn perform_insert(map: &mut HashMap<usize, &i32>, field: &mut Vec<i32>) {
//     for (key, value) in &*map {
//         print!("First empty cell at index {key}!");
//     }
//     map.clear();
// }

fn iter_lines(field: &Vec<i32>) -> bool {
    let mut line: Vec<i32> = Vec::new();

    println!("Checking all columns ... ");
    for row_num in 0..8 {
        // println!("Created following indices:");
        for i in 0..9 {
            let idx: i32 = row_num + (9 * i);
            // print!("{}", idx);
            line.push(field[idx as usize]);
        }
        // let mut line =
        // line_indices.clear();
        // println!("{:?}", line);
        if !perform_line_check(&line) {
            println!("Oh no - Column contains an error!!");
            // return Err("Rowcheck failed1");
            return false
        }
        line.clear();
    }

    // for row_num in [0,1 ] {
    //     for i in 0..8 {
    //         print!("{}", row_num + (9 * i) );
    //     }
    //     println!();
    // }
    // let mut line_indices = Vec::new();
    println!("Checking all rows... ");
    for col_num in 0..8 {
        // println!("Created following indices:");
        for i in 0..9 {
            let idx: i32 = (9 * col_num) + i;
            // print!("{}", idx);
            line.push(field[idx as usize]);
        }
        // let mut line =
        // line_indices.clear();
        // println!("{:?}", line);
        if !perform_line_check(&line) {
            println!("Oh no - Line contains an error!!");
            // return Err("Colcheck failed1");
            return false
        }
        line.clear();
    }
    // Ok((true))
    true
}

fn perform_line_check(line: &Vec<i32>) -> bool {
    // line.chunks(chunk_size)
    for el in line.iter().filter(|&n| *n != 0) {
        if line.iter().filter(|&n| *n == *el).count() > 1 {
            return false;
        }
    }
    true
}

fn iterate_blocks() {}

fn perform_block_check() {}

fn main() {
    let test = vec![
        0, 0, 0, 2, 6, 0, 7, 0, 1, 6, 8, 0, 0, 7, 0, 0, 9, 0, 1, 9, 0, 0, 0, 4, 5, 0, 0, 8, 2, 0,
        1, 0, 0, 0, 4, 0, 0, 0, 4, 6, 0, 2, 9, 0, 0, 0, 5, 0, 0, 0, 3, 0, 2, 8, 0, 0, 9, 3, 0, 0,
        0, 7, 4, 0, 4, 0, 0, 5, 0, 0, 3, 6, 7, 0, 3, 0, 1, 8, 0, 0, 0,
    ];
    let field = Sudoku {
        field: test,
        ..Default::default()
    };
    // let len = field.field.len();

    let test = vec![1, 1, 2];

    print!("{}", test.iter().count());

    solver(Box::new(field));
    // for col_num in 0..8 {
    //     for i in 0..9 {
    //         let idx: i32 = (9 * col_num) + i;
    //         print!("{}", idx);
    //         // line_indices.push(idx);
    //     }
    //     println!();
    // }

    // print!("{field}");
}
