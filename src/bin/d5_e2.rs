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

fn amount_of_ids_in_common(lhs: &Range, rhs: &Range) -> i64 {
    // First check if one range is inside the other
    if lhs.start >= rhs.start && lhs.end <= rhs.end {
        return lhs.ids_in_range();
    }

    if rhs.start >= lhs.start && rhs.end <= lhs.end {
        return rhs.ids_in_range();
    }

    // One range is not inside the other one, let's check if they overlap
    if lhs.end < rhs.start || rhs.end < lhs.start {
        // Ranges do not overlap
        return 0;
    }

    if lhs.start < rhs.start {
        return lhs.end - rhs.start + 1;
    }

    rhs.end - lhs.start + 1
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut ranges: Vec<Range> = Vec::new();
    let mut ranges_set: HashSet<Range> = HashSet::new();

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
                start: line.split("-").nth(0).unwrap().parse::<i64>().unwrap_or(0),
                end: line.split("-").nth(1).unwrap().parse::<i64>().unwrap_or(0),
            });
        } else {
            // Then process the IDs
            let id = line.parse::<i64>().unwrap_or(0);
            for range in ranges.iter() {
                if range.inside_range(id) {
                    ranges_set.insert(range.clone());
                    debug!("ID {} is fresh. It is inside {:?}", id, range);
                }
            }
        }
    }

    let mut ranges_to_analyze: Vec<Range> = ranges_set.into_iter().collect();
    debug!("Ranges to analyze: {:#?}", ranges_to_analyze);

    // NOTE: The idea here will be to merge/remove ranges until all the ranges
    // left are disjoint ranges:
    // - If one range fit inside other, remove the range
    // - If one range overlaps with other range, merge the ranges
    // - Repeat this process until all the ranges left are disjoint ranges

    loop {
        let mut all_ranges_are_disjoint = true;
        debug!("Ranges to analyze: {}", ranges_to_analyze.len());
        'outer: for x in 1..ranges_to_analyze.len() {
            for y in 0..x {
                // Check if one range is inside the other
                if ranges_to_analyze[x].start >= ranges_to_analyze[y].start
                    && ranges_to_analyze[x].end <= ranges_to_analyze[y].end
                {
                    // Range x is inside y. Remove x from ranges to analyze
                    ranges_to_analyze.remove(x);
                    all_ranges_are_disjoint = false;
                    break 'outer;
                }

                if ranges_to_analyze[y].start >= ranges_to_analyze[x].start
                    && ranges_to_analyze[y].end <= ranges_to_analyze[x].end
                {
                    // Range y is inside x. Remove y from ranges to analyze
                    ranges_to_analyze.remove(y);
                    all_ranges_are_disjoint = false;
                    break 'outer;
                }

                // Check if ranges are overlapped. If that is the case, remove both
                // ranges and add a new range which is the merge of the ranges
                // removed
                if ranges_to_analyze[x].end < ranges_to_analyze[y].start
                    || ranges_to_analyze[y].end < ranges_to_analyze[x].start
                {
                    // Ranges do not overlap
                    continue;
                }

                // Ranges are overlapped. Merge them
                let range_x = ranges_to_analyze.remove(x);
                let range_y = ranges_to_analyze.remove(y);
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
                ranges_to_analyze.push(merged_range);
                break 'outer;
            }
        }
        if all_ranges_are_disjoint {
            break;
        }
    }

    for range in ranges_to_analyze.iter() {
        total += range.ids_in_range();
    }

    debug!("Ranges to analyze: {:#?}", ranges_to_analyze);
    info!("Day 5 - Exercise 2. Result: {}", total);
    Ok(())
}
