use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut l: Vec<i32> = Vec::new();
        let mut r: Vec<i32> = Vec::new();
        // Reader is a list of number seperated by spaces. for example "1   3".  for each line split the numbers parse them and append them to the l and r
        reader.lines().flatten().for_each(|line| {
            let mut parts = line.split_whitespace();
            // parse the two numbers and append them to the l and r lists
            l.push(parts.next().unwrap().parse().unwrap());
            r.push(parts.next().unwrap().parse().unwrap());

            });
        l.sort();
        r.sort();
        // Now get the absolute value of the difference between the two lists and sum them
        let answer = l.iter().zip(r.iter()).map(|(a, b)| (a - b).abs()).sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        // l should be a list of numbers
        // r should be a count of how many times a number appears in a list
        let mut l: Vec<i32> = Vec::new();
        let mut r: HashMap<i32, i32> = HashMap::new();
        reader.lines().flatten().for_each(|line| {
            let mut parts = line.split_whitespace();
            // parse the two numbers and append them to the l and r lists
            l.push(parts.next().unwrap().parse().unwrap());
            // if the number is already in the hashmap increment the count
            // otherwise add it to the hashmap
            let count = r.entry(parts.next().unwrap().parse().unwrap()).or_insert(0);
            *count += 1;
        });
        // iterate over l, and return the sum of l[i] * r[l[i]]
        Ok(l.iter().map(|n| n * r.get(n).unwrap_or(&0)).sum())
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
