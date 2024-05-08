use serde::{Deserialize, de::{self, Deserializer}};
use std::fs;
use std::collections::HashSet;

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
    for (i, row) in game_config.sudoku.iter().enumerate().take(9) {
        for (j, cell) in row.row.iter().enumerate().take(9) {
            // Fill with the number or 0 if None
            sudoku_new[i][j] = cell.unwrap_or(0);
        }
    }
    sudoku_new
}

fn count_duplicates_in_set(numbers: &[i32]) -> usize {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for &num in numbers {
        // Ignore empty cells, assuming 0 is the empty cell marker
        if num == 0 {
            continue;
        }
        if !seen.insert(num) {
            duplicates.insert(num);
        }
    }

    duplicates.len()
}

fn count_duplicates(sudoku: &Vec<Vec<i32>>) -> usize {
    let mut total_duplicates = 0;

    // Check rows for duplicates
    for row in sudoku {
        total_duplicates += count_duplicates_in_set(row);
    }

    // Check columns for duplicates
    for col in 0..9 {
        let mut column = Vec::with_capacity(9);
        for row in 0..9 {
            column.push(sudoku[row][col]);
        }
        total_duplicates += count_duplicates_in_set(&column);
    }

    // Check 3x3 sub-grids for duplicates
    for grid_row in (0..9).step_by(3) {
        for grid_col in (0..9).step_by(3) {
            let mut sub_grid = Vec::with_capacity(9);
            for row in grid_row..grid_row + 3 {
                for col in grid_col..grid_col + 3 {
                    sub_grid.push(sudoku[row][col]);
                }
            }
            total_duplicates += count_duplicates_in_set(&sub_grid);
        }
    }

    total_duplicates
}

struct GameBoard {
    sudoku: Vec<Vec<i32>>,
    area: Vec<Area>,
    curr_iter: Vec<Vec<i32>>,
}

impl GameBoard {
    fn new(game_config: &GameBoardConfig) -> Self {
        Self {
            area: copy_areas(game_config),
            sudoku: copy_sudoku(game_config),
            curr_iter: copy_sudoku(game_config),
        }
    }

    fn print_assingment(&self) {
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

    fn print(&self) {
        for (i, row) in self.curr_iter.iter().enumerate() {
            println!(" --- --- --- --- --- --- --- --- --- ");
            print!("|");
            for (j, cell) in row.iter().enumerate() {
                if self.curr_iter[i][j] == 0 {
                    print!("   |");
                }
                else {
                    print!(" {} |", self.curr_iter[i][j]);
                }
            }
            println!("")
        }
        println!(" --- --- --- --- --- --- --- --- --- ");
    }

    pub fn print_areas(&self) {
        for sel_area in &self.area {
            for row in &sel_area.combinations {
                println!("{:?}", row);
            }

            println!("Area value: {}", sel_area.value);
        }
    }
    
    fn prepare_game(&mut self) {
        for sel_area in &mut self.area {
            let mut sum = 0;
            let mut none_count = 0;  // Counter for None values
            for (i, row) in sel_area.fields.iter().enumerate() {
                if row.len() == 2 {
                    let row_idx = row[0] as usize;
                    let col_idx = row[1] as usize;
        
                    if row_idx < self.sudoku.len() && col_idx < self.sudoku[row_idx].len() {
                        let value = &self.sudoku[row_idx][col_idx];
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

            sel_area.combinations = combinations(none_count as f32, (sel_area.value - sum) as f32);
        }
    }

    fn resolve_simple(&mut self) {
        for sel_area in &mut self.area {
            if sel_area.combinations.len() == 1 {
                for (i, row) in sel_area.fields.iter().enumerate() {
                    if row.len() == 2 {
                        let row_idx = row[0] as usize;
                        let col_idx = row[1] as usize;
            
                        if row_idx < self.curr_iter.len() && col_idx < self.curr_iter[row_idx].len() {
                            let value = &self.curr_iter[row_idx][col_idx];
                            if *value == 0 {
                                // self.sudoku[row_idx][col_idx] = sel_area.combinations[0][0];
                                self.curr_iter[row_idx][col_idx] = sel_area.combinations[0][0];
                            }
                        } else {
                            println!("Indices [{}, {}] are out of bounds.", row_idx, col_idx);
                        }
                    } else {
                        println!("Invalid indices format.");
                    }
                }

            }
        }
        self.area.retain(|area| area.combinations.len() != 1);
    }

    fn set_init(&mut self) {
        for sel_area in &self.area {
            let mut cloned_combinations = sel_area.combinations[0].clone();
            for (i, row) in sel_area.fields.iter().enumerate() {
                if row.len() == 2 {
                    let row_idx = row[0] as usize;
                    let col_idx = row[1] as usize;
        
                    if row_idx < self.curr_iter.len() && col_idx < self.curr_iter[row_idx].len() {
                        let value = &self.curr_iter[row_idx][col_idx];
                        if *value == 0 {
                            if let Some(last) = cloned_combinations.pop() {
                                self.curr_iter[row_idx][col_idx] = last;
                            } else {
                                println!("No elements to pop.");
                            }
                        }
                    } else {
                        println!("Indices [{}, {}] are out of bounds.", row_idx, col_idx);
                    }
                } else {
                    println!("Invalid indices format.");
                }
            }
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

    // gameboardconfig.print();

    let mut gameboard = GameBoard::new(&gameboardconfig);

    // let matrix_test = combinations(3.0, 15.0);

    // for row in &matrix_test {
    //     println!("{:?}", row);
    // }

    // println!("{:?}", matrix_test.len());

    gameboard.print_assingment();

    gameboard.prepare_game();
    gameboard.resolve_simple();
    gameboard.set_init();
    gameboard.print();
    gameboard.print_areas();

    println!("Duplicates: {}", count_duplicates(&gameboard.curr_iter));
    println!("Duplicates: {}", count_duplicates(&gameboard.sudoku));
}
