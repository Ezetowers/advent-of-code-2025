use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    let mut amount_problems = 0;
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        // First check if we are in the last line, which is the line
        // that has the operations. Store them and break the loop in
        // this case
        if line.starts_with('*')
            || line.starts_with('+')
            || line.starts_with('-')
            || line.starts_with('/')
        {
            operations = line
                .split_whitespace()
                .map(|x| x.chars().nth(0).unwrap())
                .collect();
            break;
        }

        // We are not in the last line, read every line and store
        // evey element in a different array by its index: number
        // in index 0 belongs to problem 0 to resolve
        let elements: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap_or(0))
            .collect();

        // One shot if: we do not know how many problems do we have
        // until we read at least one line. When we have this info,
        // proceed to properly create the array of problems were the
        // elements of every problem are going to be stored
        if amount_problems == 0 {
            amount_problems = elements.len();
            for _ in 0..amount_problems {
                problems.push(Vec::new());
            }
        }

        // Store the elements in every problem array
        for index in 0..amount_problems {
            problems[index].push(elements[index]);
        }

        debug!("{:#?}", elements);
        trace!("Line: {}", line);
    }

    // Input has been parsed and now we have an array of problems
    // and an array of operations. Iterate the array of problems
    // and apply the operation stored in the array of operations
    // Indexes will match
    for index in 0..amount_problems {
        let mut problem_total = problems[index][0];
        for elem_index in 1..problems[index].len() {
            match operations[index] {
                '*' => problem_total *= problems[index][elem_index],
                '+' => problem_total += problems[index][elem_index],
                '-' => problem_total -= problems[index][elem_index],
                '/' => problem_total /= problems[index][elem_index],
                _ => panic!("This should not happen"),
            }
        }
        debug!(
            "Problem: {:?} - Operation: {} - Total: {}",
            problems[index], operations[index], problem_total,
        );
        total += problem_total;
    }

    info!("Day 6 - Exercise 1. Result: {}", total);
    Ok(())
}
