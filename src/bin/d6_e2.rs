use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    // The problem with the exercise is parsing the input. Reading data in columns
    // makes no sense, so I will transpose the file in the first iteration
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut amount_rows = 0;
    for line in reader.lines() {
        let line = line?;
        input.push(line.chars().collect());
        if amount_rows == 0 {
            amount_rows = line.len();
        }
    }

    // File is transposed. If I iterate it now in columns first and then
    // in rows (upside down direction to read the rows like a cephalopod)
    // then we have basically the problem solved!. Other important things to mention:
    // - Blank lines separate problems
    // - First line in the problem has a number of the problem and the operation to apply
    debug!("Input - Amount of rows: {}", amount_rows);
    let mut problem: Vec<u64> = Vec::new();
    let mut operation: char = ' ';

    for x in 0..amount_rows {
        let mut row = String::new();
        let mut check_blank_line_found = true;
        for y in 0..input.len() {
            if input[y][x] != ' ' {
                check_blank_line_found = false;
            }
            row.push(input[y][x]);
        }

        if check_blank_line_found {
            // Problem definition has finished, populate the
            // data arrays
            problems.push(problem);
            operations.push(operation);
            problem = Vec::new();

            // NOTE: Adding an empty line in debugs to properly see
            // separate problems detected
            debug!("");
            continue;
        }

        if row.ends_with('*') || row.ends_with('+') {
            operation = row.chars().last().unwrap();
            debug!("New Problem - Operation: {}", operation);
        }

        row.pop();
        debug!("{}", row);
        problem.push(row.trim().parse::<u64>().unwrap_or(0));

        // NOTE: The current implementation does not compute the last
        // Cephalopod Math problem. Check if this is the last line of
        // the file parsed and if that is the case, add the problem
        // to the problem list (same for the operation)
        if x == amount_rows - 1 {
            problems.push(problem);
            operations.push(operation);
            problem = Vec::new();
            debug!("");
        }
    }

    // Input has been parsed and now we have an array of problems
    // and an array of operations. Iterate the array of problems
    // and apply the operation stored in the array of operations
    // Indexes will match
    for index in 0..problems.len() {
        let mut problem_total = problems[index][0];
        for elem_index in 1..problems[index].len() {
            match operations[index] {
                '*' => problem_total *= problems[index][elem_index],
                '+' => problem_total += problems[index][elem_index],
                _ => panic!("This should not happen"),
            }
        }
        debug!(
            "Operation: {} - Problem: {:?} - Total: {}",
            operations[index], problems[index], problem_total,
        );
        total += problem_total;
    }
    info!("Day 6 - Exercise 2. Result: {}", total);
    Ok(())
}
