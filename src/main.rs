use serde::{Deserialize, de::{self, Deserializer}};
use std::fs;

#[derive(Debug, Deserialize)]
struct GameBoard {
    sudoku: Vec<SudokuRow>,
    area: Vec<Area>,
}

impl GameBoard {
    fn print(&self) {
        for (i, row) in self.sudoku.iter().enumerate() {
            println!(" --- --- --- --- --- --- --- --- --- ");
            print!("|");
            for (j, cell) in row.row.iter().enumerate() {
                match cell {
                    Some(num) => print!(" {} |", num),
                    None => print!("   |"),
                }
            }
            println!("")
        }
        println!(" --- --- --- --- --- --- --- --- --- ");
    }
    
    fn prepare_game(&self) {
        for sel_area in &self.area {
            let mut sum = 0;
            let mut none_count = 0;  // Counter for None values
            for (i, row) in sel_area.fields.iter().enumerate() {
                if row.len() == 2 {
                    let row_idx = row[0] as usize;
                    let col_idx = row[1] as usize;
        
                    // Check if the indices are within the bounds of the sudoku grid
                    if row_idx < self.sudoku.len() && col_idx < self.sudoku[row_idx].row.len() {
                        // Access the element safely
                        let value = &self.sudoku[row_idx].row[col_idx];
                        println!("Value at [{}, {}]: {:?}", row_idx, col_idx, value);
                        // Check if the value is None and increment none_count if true
                        if value.is_none() {
                            none_count += 1;
                        }

                        // Add value to sum, treating None as 0
                        sum += value.unwrap_or(0);
                    } else {
                        println!("Indices [{}, {}] are out of bounds.", row_idx, col_idx);
                    }
                } else {
                    println!("Invalid indices format.");
                }
            }

            let sel_area_combinations = combinations(none_count as f32, (sel_area.value - sum) as f32);

            for row in &sel_area_combinations {
                println!("{:?}", row);
            }

            println!("Area value: {}", sel_area.value);
        }
    }
}

#[derive(Debug, Deserialize)]
struct SudokuRow {
    #[serde(deserialize_with = "parse_optional_ints")]
    row: Vec<Option<i32>>,
}

#[derive(Debug, Deserialize)]
struct Area {
    fields: Vec<Vec<i32>>,  // Each field is a pair of strings
    value: i32,
}

impl Area {
    fn combinations_all_fileds(&self) {
        // let N: usize = self.fields.len();
        // let s: i32 = self.value;
        // let combin_area: Vec<Vec<i32>>;
        // let upper_bound: f32 = s-(N-1)*(10-0.5*N);
        // for i in self.fields.iter().enumerate() {
        //     self.fields[i];
        // }
    }

    fn print_coords(&self) {
        for (i, row) in self.fields.iter().enumerate() {
            // Iterating over each integer in the sub-vector
            for (j, value) in row.iter().enumerate() {
                println!("Element at [{}][{}] is {}", i, j, value);
            }
        }
    }
}

fn remove_duplicates(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Filter the outer vector, retaining only those inner vectors without duplicates
    a.into_iter()
     .filter(|inner_vec| {
         // Create a HashSet to track seen elements
         let mut seen = std::collections::HashSet::new();
         // Check each element in the inner vector, ensuring it's not a duplicate
         inner_vec.iter().all(|&item| seen.insert(item))
     })
     .collect()
}

fn combinations(n: f32, s: f32) -> Vec<Vec<i32>> {
    let mut combin_area = Vec::new();

    if n!=0.0 {
        let lower_bound_raw: f32 = s-(n-1.0)*(10.0-0.5*n);
        let upper_bound_raw: f32 = s-0.5*n*(n-1.0);
    
        let lower_bound = lower_bound_raw.max(1.0).ceil() as i32;
        let upper_bound = upper_bound_raw.max(0.0).min(9.0).floor() as i32;
    
        println!("Up raw {}, down raw {}", upper_bound_raw, lower_bound_raw);
        println!("Up {}, down {}", upper_bound, lower_bound);
    
        if lower_bound <= upper_bound {
            for i in lower_bound..=upper_bound {
                // let mut combin_area = combinations(n - 1.0, s - 1.0);
                // Create a vector with a single element i
                let mut combin_part = combinations(n - 1.0, s - i as f32);
                
                if n==1.0 {
                    let combin = vec![i];
                    combin_part.push(combin);
                }
                else if combin_part.is_empty() {
                    continue;
                }
                else {
                    for inner_vec in combin_part.iter_mut() {
                        inner_vec.push(i);
                    }
                }
                
                combin_area.append(&mut combin_part);
            }
        }
        remove_duplicates(combin_area)
    }
    else {

        combin_area
    }
}

// Deserialize function helper for SudokuRow to handle optional integers
fn parse_optional_ints<'de, D>(deserializer: D) -> Result<Vec<Option<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec_of_strings: Vec<String> = Vec::deserialize(deserializer)?;
    let parsed = vec_of_strings
        .into_iter()
        .map(|s| s.parse::<i32>().ok())
        .collect();
    Ok(parsed)
}

fn main() {
    // Load the TOML configuration file
    let gameboard_str = fs::read_to_string("config/test.toml")
        .expect("Failed to read configuration file");

    // Deserialize the TOML string into the GameBoard struct
    let gameboard: GameBoard = toml::from_str(&gameboard_str)
        .expect("Failed to parse configuration file");

    // Now you can access your configuration data
    println!("{:?}", gameboard);

    gameboard.print();

    // let matrix_test = combinations(3.0, 15.0);

    // for row in &matrix_test {
    //     println!("{:?}", row);
    // }

    // println!("{:?}", matrix_test.len());

    gameboard.prepare_game();
}
