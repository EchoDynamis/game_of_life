use std::io::{self, Write};
use std::{thread, time};

const WIDTH: usize = 30;
const HEIGHT: usize = 20;

fn main() {
    let mut grid = [[false; WIDTH]; HEIGHT];
    // Initialize with a glider
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        display(&grid);
        grid = next_generation(&grid);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn display(grid: &[[bool; WIDTH]; HEIGHT]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { "█" } else { " " });
        }
        println!();
    }
}

fn next_generation(grid: &[[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut new_grid = [[false; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let neighbors = count_neighbors(grid, i, j);
            new_grid[i][j] = match (grid[i][j], neighbors) {
                (true, 2) | (true, 3) | (false, 3) => true,
                _ => false,
            };
        }
    }
    new_grid
}

fn count_neighbors(grid: &[[bool; WIDTH]; HEIGHT], i: usize, j: usize) -> u8 {
    let mut count = 0;
    for di in [-1, 0, 1].iter() {
        for dj in [-1, 0, 1].iter() {
            if *di == 0 && *dj == 0 {
                continue;
            }
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < HEIGHT as i32 && nj >= 0 && nj < WIDTH as i32 {
                if grid[ni as usize][nj as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}