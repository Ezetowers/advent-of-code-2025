use log::*;
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2025::common;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn inside_range(&self, number: i64) -> bool {
        number >= self.start && number <= self.end
    }

    fn ids_in_range(&self) -> i64 {
        self.end - self.start + 1
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

        if line.is_empty() {
            break;
        }

        // First read the ranges
        ranges.push(Range {
            start: line.split("-").nth(0).unwrap().parse::<i64>().unwrap_or(0),
            end: line.split("-").nth(1).unwrap().parse::<i64>().unwrap_or(0),
        });
    }

    // NOTE: The idea here will be to merge/remove ranges until all the ranges
    // left are disjoint ranges:
    // - If one range fit inside other, remove the range
    // - If one range overlaps with other range, merge the ranges
    // - Repeat this process until all the ranges left are disjoint ranges

    loop {
        debug!("Amount of Ranges to analyze: {}", ranges.len());
        // debug!("Ranges to analyze: {:#?}", ranges);
        let mut all_ranges_are_disjoint = true;
        'outer: for x in 1..ranges.len() {
            for y in 0..x {
                // Check if one range is inside the other
                if ranges[x].start >= ranges[y].start && ranges[x].end <= ranges[y].end {
                    // Range x is inside y. Remove x from ranges to analyze
                    all_ranges_are_disjoint = false;
                    debug!(
                        "{:?} is inside {:?}. Removing duplicate range",
                        ranges[x], ranges[y]
                    );
                    ranges.remove(x);
                    break 'outer;
                }

                if ranges[y].start >= ranges[x].start && ranges[y].end <= ranges[x].end {
                    // Range y is inside x. Remove y from ranges to analyze
                    all_ranges_are_disjoint = false;
                    debug!(
                        "{:?} is inside {:?}. Removing duplicate range",
                        ranges[y], ranges[x]
                    );
                    ranges.remove(y);
                    break 'outer;
                }

                // Check if ranges are overlapped. If that is the case, remove both
                // ranges and add a new range which is the merge of the ranges
                // removed
                if ranges[x].end < ranges[y].start || ranges[y].end < ranges[x].start {
                    // Ranges do not overlap
                    continue;
                }

                // Ranges are overlapped. Merge them
                let range_x = ranges.remove(x);
                let range_y = ranges.remove(y);
                let merged_range;
                if range_x.start <= range_y.start {
                    merged_range = Range {
                        start: range_x.start,
                        end: range_y.end,
                    };
                } else {
                    merged_range = Range {
                        start: range_y.start,
                        end: range_x.end,
                    };
                }
                all_ranges_are_disjoint = false;
                ranges.push(merged_range.clone());
                debug!(
                    "Merging {:?} and {:?}. New {:?}",
                    range_x, range_y, merged_range
                );
                break 'outer;
            }
        }
        if all_ranges_are_disjoint {
            break;
        }
    }

    for range in ranges.iter() {
        debug!("{:?} - Amount IDs: {}", range, range.ids_in_range());
        total += range.ids_in_range();
    }

    info!("Day 5 - Exercise 2. Result: {}", total);
    Ok(())
}
