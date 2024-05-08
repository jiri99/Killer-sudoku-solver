use serde::{Deserialize, de::{self, Deserializer}};
use std::{fs, process::id};
use std::collections::HashSet;
use rand::Rng;

mod game_config;
use game_config::GameBoardConfig;

mod area;
use area::{Area, combinations};

use plotters::prelude::*;


fn plot_graph(x: Vec<i32>, y: Vec<i32>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the input vectors have the same length
    assert_eq!(x.len(), y.len(), "x and y vectors must have the same length");

    // Find the range for the x and y axes
    let (x_min, x_max) = (*x.iter().min().unwrap(), *x.iter().max().unwrap());
    let (y_min, y_max) = (*y.iter().min().unwrap(), *y.iter().max().unwrap());

    // Create a drawing area for the plot
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Configure and draw the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Plot of x vs. y", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    // Create a vector of (x, y) tuples
    let data: Vec<(i32, i32)> = x.into_iter().zip(y.into_iter()).collect();

    // Plot the data as a line series
    chart.draw_series(LineSeries::new(data, &BLUE))?;

    root.present()?;
    Ok(())
}

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

fn copy_raw_sudoku(game_config: &GameBoardConfig) -> Vec<Vec<i32>> {
    let mut sudoku_new: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
    for (i, row) in game_config.sudoku.iter().enumerate().take(9) {
        for (j, cell) in row.row.iter().enumerate().take(9) {
            // Fill with the number or 0 if None
            sudoku_new[i][j] = cell.unwrap_or(0);
        }
    }
    sudoku_new
}

fn copy_sudoku(sudoku: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sudoku_new: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
    for (i, row) in sudoku.iter().enumerate().take(9) {
        for (j, cell) in row.iter().enumerate().take(9) {
            // Fill with the number or 0 if None
            sudoku_new[i][j] = *cell;
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

fn check_conflict_with_init(sel_area: &mut Area, sudoku: Vec<Vec<i32>>) {
    let area_comb_copy = sel_area.combinations.clone();
    let mut id_to_remove = Vec::new();
    for (i, row) in area_comb_copy.iter().enumerate() {
        let mut sudoku_check = copy_sudoku(&sudoku);
        let mut cloned_combinations = area_comb_copy[i].clone();
        for (j, row) in sel_area.fields.iter().enumerate() {
            if row.len() == 2 {
                let row_idx = row[0] as usize;
                let col_idx = row[1] as usize;
    
                if row_idx < sudoku_check.len() && col_idx < sudoku_check[row_idx].len() {
                    let value = &sudoku_check[row_idx][col_idx];
                    if *value == 0 {
                        if let Some(last) = cloned_combinations.pop() {
                            sudoku_check[row_idx][col_idx] = last;
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

        let dup_count = count_duplicates(&sudoku_check);

        if dup_count != 0 {
            id_to_remove.push(i);
        }
    }
    println!("Duplicates count: {:?}, value: {}", id_to_remove, sel_area.value);

    while id_to_remove.len() > 0 {
        if let Some(remove_id) = id_to_remove.pop() {
            sel_area.combinations.remove(remove_id);
        } else {
            println!("No elements to pop.");
        }
    }
}

struct GameBoard {
    sudoku: Vec<Vec<i32>>,
    area: Vec<Area>,
    curr_iter: Vec<Vec<i32>>,
    best_iter: Vec<Vec<i32>>,
    energy_x: Vec<i32>,
    energy_y: Vec<i32>,
}

impl GameBoard {
    fn new(game_config: &GameBoardConfig) -> Self {
        Self {
            area: copy_areas(game_config),
            sudoku: copy_raw_sudoku(game_config),
            curr_iter: copy_raw_sudoku(game_config),
            best_iter: copy_raw_sudoku(game_config),
            energy_x: Vec::new(),
            energy_y: Vec::new(),
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

    fn print_solution(&self) {
        for (i, row) in self.best_iter.iter().enumerate() {
            println!(" --- --- --- --- --- --- --- --- --- ");
            print!("|");
            for (j, cell) in row.iter().enumerate() {
                if self.best_iter[i][j] == 0 {
                    print!("   |");
                }
                else {
                    print!(" {} |", self.best_iter[i][j]);
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

    fn reduce_combinations(&mut self) {
        // check_conflict_with_init(sel_area, copy_sudoku(&self.sudoku));
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

    fn swap(&self) -> Vec<Vec<i32>> {
        let mut sudoku_neighbor: Vec<Vec<i32>> = copy_sudoku(&self.curr_iter);

        let mut rng_int = rand::thread_rng();
        let area_id_swap = self.area.len() as u32;
        let random_area: usize = rng_int.gen_range(0..area_id_swap) as usize;

        let comb_id_swap = self.area[random_area].combinations.len() as u32;
        let random_comb: usize = rng_int.gen_range(0..comb_id_swap) as usize;

        let mut cloned_combinations = self.area[random_area].combinations[random_comb].clone();
        for (i, row) in self.area[random_area].fields.iter().enumerate() {
            if row.len() == 2 {
                let row_idx = row[0] as usize;
                let col_idx = row[1] as usize;
    
                if row_idx < self.curr_iter.len() && col_idx < self.curr_iter[row_idx].len() {
                    let value = &self.sudoku[row_idx][col_idx];
                    if *value == 0 {
                        if let Some(last) = cloned_combinations.pop() {
                            sudoku_neighbor[row_idx][col_idx] = last;
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

        sudoku_neighbor
    }

    fn simulated_annealing(&mut self) {
        // random number generator
        let mut rng = rand::thread_rng();

        let mut temp: f32 = 1.0;
        let temp_end: f32 = 0.01;
        let alpha: f32 = 0.99;
        self.set_init();

        self.best_iter = copy_sudoku(&self.curr_iter);
        let mut best_energy: i32 = count_duplicates(&self.curr_iter) as i32;

        let mut iteration_counter: i32 = 0;
        while temp > temp_end {
            for _ in 0..100 {
                let new_board: Vec<Vec<i32>> = self.swap();
                let new_energy: i32 = count_duplicates(&new_board) as i32;
                let delta_energy: i32 = new_energy - (count_duplicates(&self.curr_iter) as i32);
                if (delta_energy < 0) || ((- (delta_energy as f32) / temp).exp() > rng.gen_range(0.0..1.0)) {
                    self.curr_iter = copy_sudoku(&new_board);
                    if new_energy < best_energy {
                        best_energy = new_energy;
                        self.best_iter = copy_sudoku(&new_board);
                    }
                }
                self.energy_x.push(iteration_counter);
                self.energy_y.push(new_energy);
                iteration_counter += 1;
            }
            temp *= alpha;
        }
    }
}

fn main() {
    // Load the TOML configuration file
    // let filename: String = "config/test_easy.toml".to_string();
    let filename: String = "config/test_medium.toml".to_string();

    let gameboard_str = fs::read_to_string(filename)
        .expect("Failed to read configuration file");

    // Deserialize the TOML string into the GameBoard struct
    let gameboardconfig: GameBoardConfig = toml::from_str(&gameboard_str)
        .expect("Failed to parse configuration file");

    let mut gameboard = GameBoard::new(&gameboardconfig);

    // Print of original sudoku
    gameboard.print_assingment();

    // Prepare game and resolve simple areas
    gameboard.prepare_game();
    gameboard.resolve_simple();
    
    // Print of prepared sudoku
    gameboard.print();

    println!("Simulated annealing start.");
    // Simulated annealing
    gameboard.simulated_annealing();

    // Print of best solution
    gameboard.print_solution();

    println!("Duplicates in best solution {}", count_duplicates(&gameboard.best_iter));
    
    if let Err(e) = plot_graph(gameboard.energy_x, gameboard.energy_y, "plots/graph_energy.png") {
        println!("Error plotting the graph: {}", e);
    } else {
        println!("Graph plotted successfully in graph.png");
    }
}
