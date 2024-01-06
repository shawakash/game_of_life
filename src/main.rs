use std::{thread, time, io::Write};

const WIDTH: usize = 50;
const HEIGHT: usize = 20;
const DELAY_MS: u64 = 100;

fn main() {
    // Set up the initial state of the grid
    let mut grid = randomize_grid();

    loop {
        // Draw the current state of the grid
        draw_grid(&grid);

        // Update the grid based on the rules of the Game of Life
        grid = update_grid(&grid);

        // Pause for a short duration to visualize the evolution
        thread::sleep(time::Duration::from_millis(DELAY_MS));

        // Clear the terminal screen for the next iteration
        print!("\x1B[2J\x1B[H");
        // flush the output to ensure the clear screen is executed
        std::io::stdout().flush().unwrap();
    }
}

fn randomize_grid() -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            grid[i][j] = rand::random();
        }
    }

    grid
}

fn draw_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &cell in row {
            if cell {
                print!("■");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn update_grid(current_grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let neighbors = count_neighbors(current_grid, i, j);

            // Apply the rules of the Game of Life
            new_grid[i][j] = match (current_grid[i][j], neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_grid
}

fn count_neighbors(grid: &Vec<Vec<bool>>, i: usize, j: usize) -> u8 {
    let mut count = 0;

    for x in (i as isize - 1)..=(i as isize + 1) {
        for y in (j as isize - 1)..=(j as isize + 1) {
            if x >= 0
                && x < HEIGHT as isize
                && y >= 0
                && y < WIDTH as isize
                && !(x == i as isize && y == j as isize)
            {
                count += grid[x as usize][y as usize] as u8;
            }
        }
    }

    count
}
