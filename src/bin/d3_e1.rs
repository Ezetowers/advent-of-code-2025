use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

/*---------------------------------------------------------------------------*/
fn find_max_in_battery(battery: &str) -> (&str, usize) {
    let mut max_value = "0";
    let mut max_index = 0;
    let curr_index = 0;

    for i in curr_index..battery.len() {
        if *max_value < battery[i..i + 1] {
            max_value = &battery[i..i + 1];
            max_index = i;
        }
    }

    (max_value, max_index)
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        trace!("Line: {}", line);

        let (max_value, max_value_index) = find_max_in_battery(&line);

        // There are two scenarios from now on
        let joltage;
        if max_value_index == line.len() - 1 {
            // First scenario: The max index is the last cell in the battery.
            // In this case, find the next maximum removing the last element
            // of the battery
            let (first_value, first_value_index) = find_max_in_battery(&line[0..line.len() - 1]);
            joltage = format!("{}{}", first_value, max_value)
                .parse::<u64>()
                .unwrap();

            debug!(
                "Battery: {} - First Value: [{}/{}] - Second Value: [{}/{}] - Joltage: {}",
                line, first_value, first_value_index, max_value, max_value_index, joltage,
            );
        } else {
            // Second scenario: The max element is not the last cell in the battery.
            // In this case, find the next maximum finding the max value in the
            // elements of the battery that are to the right of the max value
            let (second_value, second_value_index) =
                find_max_in_battery(&line[max_value_index + 1..line.len()]);
            joltage = format!("{}{}", max_value, second_value)
                .parse::<u64>()
                .unwrap();

            debug!(
                "Battery: {} - First Value: [{}/{}] - Second Value: [{}/{}] - Joltage: {}",
                line, max_value, max_value_index, second_value, second_value_index, joltage
            );
        }
        total += joltage;
    }

    info!("Day 3 - Exercise 1. Result: {}", total);
    Ok(())
}
