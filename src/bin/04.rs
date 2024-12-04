use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn get_chars(board: Vec<Vec<char>>, wrds: Vec<(usize,usize)>) -> Option<Vec<char>> {
        let mut chars = Vec::new();
        for (x, y) in wrds {
            if let Some(c) = board.get(y).and_then(|row| row.get(x)) {
                chars.push(*c);
            } else {
                return None;
            }
        }
        Some(chars)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0;
        let board: Vec<Vec<char>> = reader.lines().flatten().map(|line| line.chars().collect()).collect();


        fn get_words(board: Vec<Vec<char>>, x:usize, y:usize) -> Vec<Vec<char>> {
            let mut pts = vec![
                vec![(x, y+1), (x, y+2), (x, y+3)],
                vec![(x+1, y), (x+2, y), (x+3, y)],
                vec![(x+1, y+1), (x+2, y+2), (x+3, y+3)],
            ];
            if  y>=3 {
                pts.push(vec![(x, y-1), (x, y-2), (x, y-3)]);
                pts.push(vec![(x+1, y-1), (x+2, y-2), (x+3, y-3)]);
            }
            if x>=3 {
                pts.push(vec![(x-1, y), (x-2, y), (x-3, y)]);
                pts.push(vec![(x-1, y+1), (x-2, y+2), (x-3, y+3)]);
            }
            if x>=3 && y>=3 {
                pts.push(vec![(x-1, y-1), (x-2, y-2), (x-3, y-3)]);
            }

            pts.into_iter().filter_map(|wrds| get_chars(board.clone(), wrds)).collect()
        }

        for (y, row) in board.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'X' {
                    let words = get_words(board.clone(), x, y);
                    for word in words {
                        let s: String = word.iter().collect();
                        if s == "MAS"  {
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0;
        let board: Vec<Vec<char>> = reader.lines().flatten().map(|line| line.chars().collect()).collect();
        fn get_words(board: Vec<Vec<char>>, x:usize, y:usize) -> Vec<String> {
            if x<1 || y<1 {
                return Vec::new();
            };
            let pts = vec![
                vec![(x+1,y-1), (x-1, y+1)],
                vec![(x+1, y+1), (x-1, y-1)],
            ];
            let chars: Vec<Vec<char>>= pts.into_iter().filter_map(|wrds| get_chars(board.clone(), wrds)).collect();
            let chars = chars.iter().map(|chars| chars.iter().collect()).collect();
            chars
        }

        for (y, row) in board.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'A' {
                    let words = get_words(board.clone(), x, y);
                    if words.len()==2 && words.iter().all(|w| w =="MS" || w == "SM") {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
