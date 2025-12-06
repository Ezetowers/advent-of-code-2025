use log::*;
use std::collections::HashMap;
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
        // Calculate the intervals (until 20) in which we can split the numbers to analyze
        // Example:
        // 10: can be split in groups of 2 and 5, since those numbers are multiple. 1 and 10
        // are also multiples of 10 but those makes no sense since this case needs to be
        // tested for every number: check if all the digits in the number are the same
        let mut multiple_decomposition: HashMap<u32, Vec<u32>> = HashMap::new();
        for i in 2..=20 {
            let mut new_vec: Vec<u32> = Vec::new();
            for j in 2..i {
                if i % j == 0 {
                    new_vec.push(j);
                }
            }
            if new_vec.len() > 1 {
                multiple_decomposition.insert(i, new_vec);
            } else {
                // If the number has a prime number of digits, the only way to check if all
                // the digits are the same is checking if every digit is the same. In this
                // case, add the multiple 1
                new_vec.push(1);
                multiple_decomposition.insert(i, new_vec);
            }
        }
        trace!("Multiple Decomposition: {:#?}", multiple_decomposition);
        let mut sum = 0;

        //
        for value in self.initial..=self.end {
            let amount_digits = count_digits(value.try_into().unwrap());
            sum += get_invalid_ids_string_version(
                value,
                amount_digits,
                multiple_decomposition.get(&amount_digits).unwrap(),
            );
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

/// Invalid IDs will be calculated converting the number to string and then subdividing
/// the string into chunks that are multiples of the amount of digits of the number:
/// Example with 12341234:
/// - This number has 8 digits, which means we need we need to split the number
///   in chunk of size 4 or 2 (multiples of 8).
/// - If we check chunks of size 4, we need to compare 1234 vs 1234. They are equal, which
///   means the number is an invalid id
/// - If we ccheck chunks of 2, we need to compare 12 vs 34 vs 12 vs 34. In this case
///   the chunks *are not equal*. Here it can be seen than the second comparison is not
///   needed, which means an optimization that can be done is to just finish the method
///   when we find that all the chunks are the same in any multiple
fn get_invalid_ids_string_version(number: u64, digits: u32, multiples: &Vec<u32>) -> u64 {
    let num_to_string = number.to_string();

    for multiple in multiples.iter() {
        let mut valid_id_found = false;
        for i in 0..digits / multiple - 1 {
            let first_index = i as usize * *multiple as usize;
            let second_index = (i + 1) as usize * *multiple as usize;
            let third_index = (i + 2) as usize * *multiple as usize;
            let first_value = &num_to_string[first_index..second_index];
            let second_value = &num_to_string[second_index..third_index];
            debug!(
                "Number: {} - Digits: {} - Chunk: {} - Multiple: {} - First value: {} - Second value: {}",
                number, digits, i, multiple, first_value, second_value
            );

            if first_value != second_value {
                valid_id_found = true;
                break;
            }
        }
        if !valid_id_found {
            // If we reach here, then all the chunks are identical of the multiple are identical.
            // Break the for and return the number as the invalid ID
            info!(
                "Number {} is an invalid ID. Multiple in which was detected: {}",
                number, multiple
            );
            return number;
        }
    }
    0
}

fn get_invalid_ids_multiple_version(number: u64, digits: u32, multiples: &Vec<u32>) -> u64 {
    number
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
        info!("Processing range: {:#?}", range);
        total += range.calculate_duplicates_sum();
    }

    info!("Day 2 - Exercise 2. Result: {}", total);
    Ok(())
}
