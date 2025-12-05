use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut dial = 50;
    let mut zeros_found = 0;
    let mut rotations_found = 0;
    let mut total = 0;
    const MAX: i32 = 100;

    for line in reader.lines() {
        let mut line = line?;
        trace!("Line: {}", line);

        // Parse the input to get the direction and value to rotate the dial
        let direction = line.chars().nth(0).unwrap();
        line.remove(0);
        let value = line.parse::<i32>().unwrap_or(0);

        let previous_dial = dial;
        if direction == 'L' {
            dial -= value;
            while dial < 0 {
                rotations_found += 1;
                dial += MAX;
            }

            // Special case - If we want to go left when the previous dial
            // is 0, we need to avoid adding ONE rotation (the first one
            // when we go to the left. Examples:
            // * L50 -> L15 (output: 1 // only a zero)
            // * L50 -> L115 (output: 2 // one rotation, one zero)
            if previous_dial == 0 {
                rotations_found -= 1;
            }
        } else {
            dial += value;
            while dial > MAX {
                rotations_found += 1;
                dial -= MAX;
            }
        }

        if dial == MAX {
            dial = 0;
        }

        if dial == 0 {
            zeros_found += 1;
        }

        if dial == MAX {
            dial = 0;
        }

        total = zeros_found + rotations_found;
        debug!(
            "Direction: {} - Value: {} - Dial after move: {} - Zeros found: {} - Rotations found: {} - Total: {}",
            direction, value, dial, zeros_found, rotations_found, total
        );
    }

    info!("Day 1 - Exercise 2. Result: {}", total);
    Ok(())
}
