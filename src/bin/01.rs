use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use priority_queue::PriorityQueue;
use adv_code_2024::*;
use dary_heap::DaryHeap;


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

    fn part1_pq<R: BufRead>(reader: R) -> Result<i32> {
        let mut l = PriorityQueue::new();
        let mut r = PriorityQueue::new();
        reader.lines().flatten().enumerate().for_each(|(i,line)| {
            let mut parts = line.split_whitespace();
            // parse the two numbers and append them to the l and r lists
            let left: i32 = parts.next().unwrap().parse().unwrap();
            let right: i32 = parts.next().unwrap().parse().unwrap();
            l.push(i, left);
            r.push(i, right);
        });
        let answer = l.into_sorted_iter().zip(r.into_sorted_iter()).map(|((_,a), (_,b))| (a - b).abs()).sum();
        Ok(answer)
    }
    fn part1_heap<const N:usize, R: BufRead>(reader: R) -> Result<i32> {
        let mut l = DaryHeap::<_, N>::new();
        let mut r = DaryHeap::<_, N>::new();
        reader.lines().flatten().enumerate().for_each(|(i,line)| {
            let mut parts = line.split_whitespace();
            // parse the two numbers and append them to the l and r lists
            let left: i32 = parts.next().unwrap().parse().unwrap();
            let right: i32 = parts.next().unwrap().parse().unwrap();
            l.push(left);
            r.push(right);
        });
        let answer = l.into_iter_sorted().zip(r.into_iter_sorted()).map(|(a,b)| (a - b).abs()).sum();
        Ok(answer)
    }


assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    println!("\n=== Priority Queue ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1_pq(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Heap 2 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1_heap::<2,_>(input_file)?);
    println!("Result = {}", result);
    println!("\n=== Heap 5 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1_heap::<5,_>(input_file)?);
    println!("Result = {}", result);
    println!("\n=== Heap 10 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1_heap::<10,_>(input_file)?);
    println!("Result = {}", result);

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
