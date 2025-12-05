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
    const MAX: i32 = 100;

    for line in reader.lines() {
        let mut line = line?;

        // Parse the input to get the direction and value to rotate the dial
        let direction = line.chars().nth(0).unwrap();
        line.remove(0);
        let value = line.parse::<i32>().unwrap_or(0);

        if direction == 'L' {
            dial -= value;
            while dial < 0 {
                dial += MAX;
            }
        } else {
            dial += value;
            while dial >= MAX {
                dial -= MAX;
            }
        }

        if dial == 0 {
            zeros_found += 1;
        }

        trace!("Line: {}", line);
        debug!(
            "Direction: {} - Value: {} - Dial after move: {} - Zeros found: {}",
            direction, value, dial, zeros_found
        );
    }

    info!("Day 1 - Exercise 1. Result: {}", zeros_found);
    Ok(())
}
