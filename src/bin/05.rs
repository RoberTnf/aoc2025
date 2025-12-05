#![feature(step_trait)]

use std::{collections::HashSet, io::Read, iter::Step};

fn main() {
    let data = load_data("data/05.txt").unwrap();
    println!("{}", solve_first(data.1, data.0.clone()));
    println!("{}", solve_second(data.0));
}

#[derive(Clone, Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, number: u64) -> bool {
        self.start <= number && self.end >= number
    }

    fn all(&self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        for i in self.start..=self.end {
            result.push(i);
        }
        result
    }

    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

fn load_data(fname: &str) -> Result<(Vec<Range>, Vec<u64>), std::io::Error> {
    let mut file = std::fs::File::open(fname)?;
    let mut text = String::new();

    let mut ids: Vec<u64> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();
    file.read_to_string(&mut text)?;

    text.lines().for_each(|line| {
        let parts = line.trim().split('-').collect::<Vec<&str>>();
        if line.is_empty() {
            return;
        }

        match parts.len() {
            0 => {}
            1 => ids.push(parts[0].parse::<u64>().unwrap()),
            2 => {
                let start = parts[0].parse::<u64>().unwrap();
                let end = parts[1].parse::<u64>().unwrap();
                ranges.push(Range { start, end });
            }
            _ => {}
        }
    });
    Ok((ranges, ids))
}

fn solve_first(ids: Vec<u64>, ranges: Vec<Range>) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(**id)))
        .count()
}

fn solve_second(ranges: Vec<Range>) -> usize {
    let condensed_ranges = recursively_condense(ranges);
    condensed_ranges.iter().map(|range| range.len()).sum()
}

fn recursively_condense(ranges: Vec<Range>) -> Vec<Range> {
    let num_original_ranges = ranges.len();
    let mut condensed_ranges: Vec<Range> = Vec::new();
    ranges.iter().for_each(|range| {
        for cd in condensed_ranges.iter_mut() {
            // check if ranges overlap
            if (range.start <= cd.end && range.end >= cd.start)
                || (cd.start <= range.end && cd.end >= range.start)
            {
                cd.start = range.start.min(cd.start);
                cd.end = range.end.max(cd.end);
                return;
            }
        }
        condensed_ranges.push(range.clone());
    });
    if condensed_ranges.len() == num_original_ranges {
        return condensed_ranges;
    }
    recursively_condense(condensed_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let (ranges, ids) = load_data("data/05_test.txt").unwrap();
        assert_eq!(ranges.len(), 4);
        assert_eq!(ids.len(), 6);
    }

    #[test]
    fn test_solve_first() {
        let (ranges, ids) = load_data("data/05_test.txt").unwrap();
        assert_eq!(solve_first(ids, ranges), 3);
    }

    #[test]
    fn test_solve_second() {
        let ranges = load_data("data/05_test.txt").unwrap().0;
        assert_eq!(solve_second(ranges), 14);
    }
}
