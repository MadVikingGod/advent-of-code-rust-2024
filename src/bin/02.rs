use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    struct Report(Vec<i32>);

    impl FromStr for Report {
        type Err = Error;
        fn from_str(line: &str) -> Result<Self> {
            Ok(Report::new(line))
        }
    }

    impl From<String> for Report {
        fn from(line: String) -> Self {
            Report::new(&line)
        }
    }

    impl Report {
        fn new(line: &str) -> Self {
            Report(line.split(' ').map(|s| s.parse().unwrap()).collect())
        }
        fn is_increasing(&self) -> bool {
            self.0.windows(2).all(|w| w[0] <= w[1])
        }
        fn is_decreasing(&self) -> bool {
            self.0.windows(2).all(|w| w[0] >= w[1])
        }
        fn safe_distance(&self) -> bool {
            self.0
                .windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .all(|d| d >= 1 && d <= 3)
        }
        fn is_safe(&self) -> bool {
            (self.is_increasing() || self.is_decreasing()) && self.safe_distance()
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .map(Report::from)
            .filter(|r| r.is_safe())
            .count();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn remove_one<T: Clone>(input: Vec<T>) -> Vec<Vec<T>> {
        let mut result = Vec::new();

        for i in 0..input.len() {
            let mut temp = input.clone();
            temp.remove(i);
            result.push(temp);
        }

        result
    }

    impl Report {
        fn is_safe2(&self) -> bool {
            if self.is_safe() {
                return true;
            };
            remove_one(self.0.clone())
                .iter()
                .any(|r| Report(r.clone()).is_safe())
        }
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .map(Report::from)
            .filter(|r| r.is_safe2())
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
