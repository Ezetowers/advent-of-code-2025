use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    // NOTE: Old trick: we will try to add "a frame" to the grid and then iterate it
    // from coordinates (1,1) to (n-1,n-1). Adding the frame will allow us to avoid
    // going out of bounds when we check elements on the sides of the grid
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut row_size = 0;
    for line in reader.lines() {
        let line = line?;
        trace!("Line: {}", line);
        let mut row: Vec<_> = line.chars().collect();
        row.insert(0, '|');
        row.push('|');

        if row_size == 0 {
            row_size = row.len();
        }

        grid.push(row);
    }

    // Insert the upper and lower part of the frame
    let frame_row = vec!['-'].repeat(row_size);
    let other_frame_row = frame_row.clone();
    grid.insert(0, frame_row);
    grid.push(other_frame_row);

    trace!("Grid Output");
    for line in 0..grid.len() {
        trace!("{}", grid[line].iter().collect::<String>());
    }

    let mut grid_marked = grid.clone();
    for x in 1..=row_size - 2 {
        for y in 1..=row_size - 2 {
            trace!("Position: ({} - {}) - Value: {}", x, y, grid[x][y]);
            if grid[x][y] != '@' {
                continue;
            }

            // Count adjacents
            let mut adjacent_count = 0;

            'outer: for adj_x in x - 1..=x + 1 {
                for adj_y in y - 1..=y + 1 {
                    if adj_x == x && adj_y == y {
                        continue;
                    }
                    trace!(
                        "   Position: ({},{}) - Adjacent: ({},{}) - Adjacent Value: {}",
                        x,
                        y,
                        adj_x,
                        adj_y,
                        grid[adj_x][adj_y]
                    );

                    if grid[adj_x][adj_y] == '@' {
                        adjacent_count += 1;
                    }

                    if adjacent_count == 4 {
                        break 'outer;
                    }
                }
            }

            trace!(
                " Position ({},{}) - Adjacents (at least): {}",
                x,
                y,
                adjacent_count
            );
            if adjacent_count < 4 {
                debug!(
                    "Valid Roll found: Position ({},{}) - Adjacents: {}",
                    x, y, adjacent_count
                );
                grid_marked[x][y] = 'x';
                total += 1;
            }
        }
    }

    debug!("Grid Marked Output");
    for line in 0..grid_marked.len() {
        debug!("{}", grid_marked[line].iter().collect::<String>());
        // debug!("{:?}", grid_marked[line]);
    }

    info!("Day 4 - Exercise 1. Result: {}", total);
    Ok(())
}
