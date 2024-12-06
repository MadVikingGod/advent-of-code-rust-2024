use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_BOOKS: &str = concatcp!("input/", DAY, "_books.txt");
const INPUT_ORDER: &str = concatcp!("input/", DAY, "_order.txt");

const TEST_ORDER: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

const TEST_BOOKS: &str = "\
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn parse_orders<R: BufRead>(reader: R) -> HashMap<usize, Vec<usize>> {
    let mut orders = HashMap::new();
    reader.lines().flatten().for_each(|line| {
        let mut parts = line.split('|');
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        orders.entry(a).or_insert_with(Vec::new).push(b);
    });
    orders
}

fn parse_books<R: BufRead>(reader: R) -> Vec<Vec<usize>> {
    reader
        .lines()
        .flatten()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn is_valid(orders: &HashMap<usize, Vec<usize>>, book: &Vec<usize>) -> bool {
    book.iter().rev().enumerate().all(|(i, page)| {
        let remaining = book
            .iter()
            .rev()
            .skip(i + 1)
            .cloned()
            .collect::<HashSet<usize>>();
        let before = orders.get(page).cloned().unwrap_or(Vec::new());
        !before.iter().any(|b| remaining.contains(b))
    })
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(orders: R, books: R) -> Result<usize> {
        let orders = parse_orders(orders);
        let books = parse_books(books);

        Ok(books
            .iter()
            .filter(|&book| is_valid(&orders, book))
            .map(|book| book[book.len() / 2])
            .sum())
    }

    assert_eq!(
        143,
        part1(
            BufReader::new(TEST_ORDER.as_bytes()),
            BufReader::new(TEST_BOOKS.as_bytes())
        )?
    );

    let input_books = BufReader::new(File::open(INPUT_BOOKS)?);
    let input_order = BufReader::new(File::open(INPUT_ORDER)?);
    let result = time_snippet!(part1(input_order, input_books)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(orders: R, books: R) -> Result<usize> {
        let orders = parse_orders(orders);
        let books = parse_books(books);

        Ok(books
            .iter()
            .filter(|&book| !is_valid(&orders, book))
            .map(|book| {
                let mut book = book.clone();
                book.sort_by(|a, b| {
                    if orders.get(a).unwrap_or(&Vec::new()).contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                    if orders.get(b).unwrap().contains(a) {
                        return std::cmp::Ordering::Greater;
                    }
                    std::cmp::Ordering::Equal
                });
                book
            })
            .map(|book| book[&book.len() / 2])
            .sum())
    }

    assert_eq!(
        123,
        part2(
            BufReader::new(TEST_ORDER.as_bytes()),
            BufReader::new(TEST_BOOKS.as_bytes())
        )?
    );

    let input_books = BufReader::new(File::open(INPUT_BOOKS)?);
    let input_order = BufReader::new(File::open(INPUT_ORDER)?);
    let result = time_snippet!(part2(input_order, input_books)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
