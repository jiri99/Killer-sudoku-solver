use serde::{Deserialize, de::{self, Deserializer}};
use std::fs;


#[derive(Debug, Deserialize)]
pub struct GameBoardConfig {
    pub sudoku: Vec<SudokuRowConfig>,
    pub area: Vec<AreaConfig>,
}

impl GameBoardConfig {
    pub fn print(&self) {
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
pub struct SudokuRowConfig {
    #[serde(deserialize_with = "parse_optional_ints")]
    pub row: Vec<Option<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct AreaConfig {
    pub fields: Vec<Vec<i32>>,  // Each field is a pair of strings
    pub value: i32,
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
