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

fn combinations(n: f32, s: f32) -> Vec<Vec<i32>> {
    let mut combin_area = Vec::new();
    // let combin_area: Vec<Vec<i32>>;
    let lower_bound_raw: f32 = s-(n-1.0)*(10.0-0.5*n);
    let upper_bound_raw: f32 = s-0.5*n*(n-1.0);

    let lower_bound = lower_bound_raw.max(0.0).ceil() as i32;
    let upper_bound = upper_bound_raw.max(0.0).floor() as i32;

    println!("Up raw {}, down raw {}", upper_bound_raw, lower_bound_raw);
    println!("Up {}, down {}", upper_bound, lower_bound);

    for i in lower_bound..=upper_bound {
        // Create a vector with a single element i
        let combin = vec![i];
        // Push the new vector into the outer vector
        combin_area.push(combin);
    }
    combin_area
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

    let matrix_test = combinations(2.0, 5.0);

    for row in &matrix_test {
        println!("{:?}", row);
    }
}
