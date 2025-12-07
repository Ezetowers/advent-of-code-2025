use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn inside_range(&self, number: u64) -> bool {
        number >= self.start && number <= self.end
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut ranges: Vec<Range> = Vec::new();

    let mut blank_line_found = false;
    for line in reader.lines() {
        let line = line?;
        trace!("Line: {}", line);

        if !blank_line_found {
            blank_line_found = line.is_empty();
        }

        if !blank_line_found {
            // First read the ranges
            ranges.push(Range {
                start: line.split("-").nth(0).unwrap().parse::<u64>().unwrap_or(0),
                end: line.split("-").nth(1).unwrap().parse::<u64>().unwrap_or(0),
            });
        } else {
            // Then process the IDs
            let id = line.parse::<u64>().unwrap_or(0);
            for range in ranges.iter() {
                if range.inside_range(id) {
                    debug!("ID {} is fresh. It is inside {:?}", id, range);
                    total += 1;
                    break;
                }
            }
        }
    }

    info!("Day 5 - Exercise 1. Result: {}", total);
    Ok(())
}
