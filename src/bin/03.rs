use std::fs;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let answer = re.captures_iter(input).map(|cap| {
            let l = cap[1].parse::<usize>().unwrap();
            let r = cap[2].parse::<usize>().unwrap();
            l * r
        }).sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;
    let result = time_snippet!(part1(&input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
