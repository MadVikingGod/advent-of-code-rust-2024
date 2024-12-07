use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<(usize,Vec<usize>)> = reader.lines().flatten().map(|line| {
            let mut parts = line.split(": ");
            let ans = parts.next().unwrap().parse().unwrap();
            let parts = parts.next().unwrap().split(' ').map(|s| s.parse().unwrap()).rev().collect();
            (ans, parts)
        }).collect();
        println!("len of parts {}", lines.iter().map(|(_,p)| p.len()).max().unwrap());
        fn values(parts: &mut Vec<usize>, max: &usize) -> Vec<usize> {
            let mut ans = vec![parts.pop().unwrap()];
            while let Some(v) = parts.pop() {
                ans = ans
                    .iter()
                    .map(|a| vec![a+v, a*v])
                    .flatten()
                    .filter(|a| a <= max)
                    .collect();
            }
            ans
        }

        let mut count = 0;
        let mut max =0;
        for (ans, mut parts) in lines {
            let vals = values(&mut parts, &ans);
            if vals.len() > max {
                max = vals.len();
            }
            if vals.contains(&ans) {
                count += ans;
            }
        }
        println!("max {}", max);
        Ok(count)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<(usize,Vec<usize>)> = reader.lines().flatten().map(|line| {
            let mut parts = line.split(": ");
            let ans = parts.next().unwrap().parse().unwrap();
            let parts = parts.next().unwrap().split(' ').map(|s| s.parse().unwrap()).rev().collect();
            (ans, parts)
        }).collect();

        fn concat(l:&usize, r:usize) -> usize {
            let up = match r {
                _ if r>=1000 => 10000,
                _ if r>=100 => 1000,
                _ if r>=10 => 100,
                _ => 10,
            };
            l*up + r
        }
        fn values(parts: &mut Vec<usize>, max: &usize) -> Vec<usize> {
            let mut ans = vec![parts.pop().unwrap()];
            while let Some(v) = parts.pop() {
                ans = ans
                    .iter()
                    .map(|a| vec![a+v, a*v, concat(a,v)])
                    .flatten()
                    .filter(|a| a <= max)
                    .collect();
            }
            ans
        }

        let mut count = 0;
        let mut max = 0;
        for (ans, mut parts) in lines {
            let vals = values(&mut parts, &ans);
            if vals.len() > max {
                max = vals.len();
            }
            if vals.contains(&ans) {
                count += ans;
            }
        }
        println!("max {}", max);

        Ok(count)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
