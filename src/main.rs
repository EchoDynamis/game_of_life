use crossterm::{
    cursor, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{Clear, ClearType}, ExecutableCommand
};
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

    let mut generation = 0;
    let mut paused = false;

    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    loop {
        display(&grid, generation);
        stdout.flush().unwrap();

        // Check for user input
        if crossterm::event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break; // Quit on 'q'
                } else if key.code == crossterm::event::KeyCode::Char(' ') {
                    paused = !paused; // Toggle pause on spacebar
                }
            }
        }

        if !paused {
            grid = next_generation(&grid);
            generation += 1;
        }

        thread::sleep(time::Duration::from_millis(100));
    }
    // Reset terminal
    stdout.execute(ResetColor).unwrap();
    stdout.execute(cursor::Show).unwrap();
}

fn display(grid: &[[bool; WIDTH]; HEIGHT], generation: u32) {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    stdout
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap()
        .execute(Print(format!("Game of Life - Generation: {}\n", generation)))
        .unwrap()
        .execute(SetForegroundColor(Color::DarkGrey))
        .unwrap()
        .execute(Print("Press 'q' to quit, 'space' to pause/resume\n"))
        .unwrap();

    // Top border
    stdout.execute(Print("┌")).unwrap();
    for _ in 0..WIDTH {
        stdout.execute(Print("─")).unwrap();
    }
    stdout.execute(Print("┐\n")).unwrap();

    // Grid with dotted background and colored live cells
    for row in grid.iter() {
        stdout.execute(Print("│")).unwrap();
        for &cell in row.iter() {
            if cell {
                stdout
                    .execute(SetForegroundColor(Color::Green))
                    .unwrap()
                    .execute(Print("█"))
                    .unwrap();
            } else {
                stdout.execute(Print(".")).unwrap();
            }
        }
        stdout.execute(Print("│\n")).unwrap();
    }

    // Bottom border
    stdout.execute(Print("└")).unwrap();
    for _ in 0..WIDTH {
        stdout.execute(Print("─")).unwrap();
    }
    stdout.execute(Print("┘")).unwrap();
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
