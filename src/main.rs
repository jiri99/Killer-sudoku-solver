use serde::{Deserialize, de::{self, Deserializer}};
use std::fs;

mod game_config;
use game_config::GameBoardConfig;

mod area;
use area::{Area, combinations};

fn copy_areas(game_config: &GameBoardConfig) -> Vec<Area> {
    // Initialize a new vector of Area
    let mut area_new: Vec<Area> = Vec::new();

    // Iterate through each AreaConfig in game_config.area
    for sel_area in &game_config.area {
        // Create a new Area with fields copied from AreaConfig, and combinations initialized
        let sel_area_new = Area {
            fields: sel_area.fields.clone(),
            value: sel_area.value,
            combinations: Vec::new(),  // Initialize with an empty vector
        };

        // Add the new Area to area_new
        area_new.push(sel_area_new);
    }

    area_new
}

fn copy_sudoku(game_config: &GameBoardConfig) -> Vec<Vec<i32>> {
    let mut sudoku_new: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
    // for (i, row) in game_config.sudoku.iter().enumerate() {
    //     for (j, cell) in row.row.iter().enumerate() {
    //         match cell {
    //             Some(num) => sudoku_new[i][j] = *num,
    //             None => sudoku_new[i][j] = 0,
    //         }
    //     }
    // }
    for (i, row) in game_config.sudoku.iter().enumerate().take(9) {
        for (j, cell) in row.row.iter().enumerate().take(9) {
            // Fill with the number or 0 if None
            sudoku_new[i][j] = cell.unwrap_or(0);
        }
    }
    sudoku_new
}

struct GameBoard {
    sudoku: Vec<Vec<i32>>,
    area: Vec<Area>,
}

impl GameBoard {
    fn new(game_config: &GameBoardConfig) -> Self {
        Self {
            area: copy_areas(game_config),
            sudoku: copy_sudoku(game_config),
        }
    }

    fn print(&self) {
        for (i, row) in self.sudoku.iter().enumerate() {
            println!(" --- --- --- --- --- --- --- --- --- ");
            print!("|");
            for (j, cell) in row.iter().enumerate() {
                if self.sudoku[i][j] == 0 {
                    print!("   |");
                }
                else {
                    print!(" {} |", self.sudoku[i][j]);
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
                    if row_idx < self.sudoku.len() && col_idx < self.sudoku[row_idx].len() {
                        // Access the element safely
                        let value = &self.sudoku[row_idx][col_idx];
                        println!("Value at [{}, {}]: {:?}", row_idx, col_idx, value);
                        // Check if the value is None and increment none_count if true
                        if *value == 0 {
                            none_count += 1;
                        }

                        // Add value to sum, treating None as 0
                        sum += *value;
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

fn main() {
    // Load the TOML configuration file
    let gameboard_str = fs::read_to_string("config/test_easy.toml")
        .expect("Failed to read configuration file");

    // Deserialize the TOML string into the GameBoard struct
    let gameboardconfig: GameBoardConfig = toml::from_str(&gameboard_str)
        .expect("Failed to parse configuration file");

    // Now you can access your configuration data
    // println!("{:?}", gameboard);

    gameboardconfig.print();

    let gameboard = GameBoard::new(&gameboardconfig);

    // let matrix_test = combinations(3.0, 15.0);

    // for row in &matrix_test {
    //     println!("{:?}", row);
    // }

    // println!("{:?}", matrix_test.len());

    // gameboard.prepare_game();
}
