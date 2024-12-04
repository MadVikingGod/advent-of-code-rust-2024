use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let answer = re
            .captures_iter(input)
            .map(|cap| {
                let l = cap[1].parse::<usize>().unwrap();
                let r = cap[2].parse::<usize>().unwrap();
                l * r
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(161, part1(TEST)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;
    let result = time_snippet!(part1(&input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2(input: &str) -> Result<usize> {
        let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))")?;
        let mut enabled = true;
        let mut answer: usize = 0;
        for cap in re.captures_iter(input) {
            let c = &cap[0];
            if c.eq("do()") {
                enabled = true;
            } else if c.eq("don't()") {
                enabled = false;
            } else if enabled {
                let l = cap[2].parse::<usize>()?;
                let r = cap[3].parse::<usize>()?;
                answer += l * r;
            }
        }

        Ok(answer)
    }

    assert_eq!(48, part2(TEST2)?);

    let input_file = fs::read_to_string(INPUT_FILE)?;
    let result = time_snippet!(part2(&input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
