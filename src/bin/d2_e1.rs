use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

/*---------------------------------------------------------------------------*/
#[derive(Debug, Default)]
struct Range {
    initial: u64,
    end: u64,
}

impl Range {
    fn calculate_duplicates_sum(&self) -> u64 {
        let mut sum = 0;

        for value in self.initial..=self.end {
            let amount_digits_initial = count_digits(value.try_into().unwrap());
            if amount_digits_initial % 2 != 0 {
                continue;
            }
            let left = value / 10u64.pow(amount_digits_initial / 2);
            let right = value - left * 10u64.pow(amount_digits_initial / 2);
            trace!("Value: {} - Left: {} - Right: {}", value, left, right);

            if left == right {
                debug!(
                    "Range: {:#?} - Value: {} - Left: {} - Right: {}",
                    self, value, left, right
                );
                sum += value;
            }
        }
        sum
    }
}

/*---------------------------------------------------------------------------*/

fn count_digits(number: i64) -> u32 {
    if number == 0 || number == 1 {
        return 1;
    }

    let mut counter = 1;
    let mut pow_value = <u64 as TryInto<i64>>::try_into(10u64.pow(counter)).unwrap();
    while (number - pow_value) > 0 {
        trace!("number: {} - number - pow: {}", number, number - pow_value);
        counter += 1;
        pow_value = <u64 as TryInto<i64>>::try_into(10u64.pow(counter)).unwrap();
    }
    counter
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    let mut ranges: Vec<Range> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        trace!("Line: {}", line);

        // Input: 11-22,95-115,...,123-213
        // Output: [Range{11,22}, Range{123,213}
        ranges = line
            .split(",")
            .map(|x| Range {
                initial: x.split("-").nth(0).unwrap().parse::<u64>().unwrap_or(0),
                end: x.split("-").nth(1).unwrap().parse::<u64>().unwrap_or(0),
            })
            .collect();

        // NOTE: The input only has one line
        break;
    }

    debug!("{:?}", ranges);
    for range in ranges.iter() {
        total += range.calculate_duplicates_sum();
    }

    info!("Day 2 - Exercise 1. Result: {}", total);
    Ok(())
}
