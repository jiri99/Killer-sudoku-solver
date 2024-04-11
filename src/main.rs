use serde::Deserialize;
use serde::de::{self, Deserializer};
use std::fs;

struct Field {
    ver_coord: u16, // Vertical coordinates
    hor_coord: u16, // Horizontal coordinates
}

struct Area {
    fields: Vec<Field>,
}

impl Area {
    fn fill_area(&self, id: usize) -> u16 {
        self.fields[id].ver_coord
    }
}

struct GameBoard {
    input_variable: i32,
}

// The impl block is used to define methods associated with GameBoard
impl GameBoard {
    // A method that takes an immutable reference to self
    // and returns the value of input_variable
    fn get_input_variable(&self) -> i32 {
        self.input_variable
    }

    // A method that takes a mutable reference to self and an i32 value
    // It updates the value of input_variable with the given value
    fn set_input_variable(&mut self, value: i32) {
        self.input_variable = value;
    }
}


#[derive(Debug, Deserialize)]
struct Config {
    sudoku: Vec<SudokuRow>,
}

#[derive(Debug, Deserialize)]
struct SudokuRow {
    #[serde(deserialize_with = "parse_optional_ints")]
    row: Vec<Option<i32>>,
}

// Custom deserialization function to convert strings to Option<i32>
fn parse_optional_ints<'de, D>(deserializer: D) -> Result<Vec<Option<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the input as a vector of strings
    let vec_of_strings: Vec<String> = Vec::deserialize(deserializer)?;

    // Attempt to parse each string into an i32, converting errors to None
    let parsed = vec_of_strings
        .into_iter()
        .map(|s| s.parse::<i32>().ok())
        .collect();

    Ok(parsed)
}

fn main() {
    // Load the TOML configuration file
    let config_str = fs::read_to_string("config/test.toml")
        .expect("Failed to read configuration file");

    // Deserialize the TOML string into the Config struct
    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse configuration file");

    // Now you can access your configuration data
    println!("{:?}", config);
}
