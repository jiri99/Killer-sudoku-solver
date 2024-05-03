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
    fn combinations(&self) {
        let N = self.fields.len();
        let s: i32 = self.value;
        let combin_area: Vec<Vec<i32>>;
        for i in self.fields.iter().enumerate() {

        }
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
}
