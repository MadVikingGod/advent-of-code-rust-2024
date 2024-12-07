use std::collections::{HashMap, HashSet};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader, Empty};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

// type Point = (i32, i32);
#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone)]
enum Tile {
    Full,
    Empty,
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        match c {
            '#' => Ok(Tile::Full),
            '.' => Ok(Tile::Empty),
            '^' => Ok(Tile::Empty),
            _ => Err(anyhow!("Invalid character: {}", c))
        }
    }
}

#[derive(Clone)]
struct State {
    pos: Point<i32>,
    dir: Direction,
}

fn parse<R: BufRead>(reader: R) -> Result<(Map<Tile>, State)> {
    let (map,start) = Map::parse_with_start(reader, &'^')?;
    let start = State {
        pos: start,
        dir: Direction::Up,
    };
    Ok((map, start))
}
fn repr(map: &Map<Tile>, seen:&Seen) -> String {
    let mut result = String::new();
    for y in 0..=map.max.y {
        for x in 0..=map.max.x {
            let point = (x, y).into();
            let c = match (map.get(&point), seen.map.contains_key(&point)) {
                (Some(Tile::Full),_) => '#',
                (Some(Tile::Empty), false) => '.',
                (Some(Tile::Empty), true) => 'X',
                _ => {continue},

            };
            result.push(c);
        }
        result.push('\n');
    }
    result
}


struct Seen {
    map: HashMap<Point<i32>, HashSet<Direction>>
}
impl Seen {
    fn new() -> Self {
        Seen {
            map: HashMap::new()
        }
    }
    fn insert(&mut self, state: &State) {
        self.map.entry(state.pos).or_insert_with(HashSet::new).insert(state.dir.clone());
    }
    fn contains(&self, state: &State) -> bool {
        self.map.get(&state.pos).map_or(false, |set| set.contains(&state.dir))
    }
}

fn find_path(map: &Map<Tile>, start: &State) -> (Seen, bool) {
    let mut seen = Seen::new();
    let mut state = start.clone();
    loop {
        if seen.contains(&state) {
            return (seen, false);
        }
        seen.insert(&state);
        let next = match state.dir {
            Direction::Up => state.pos + Point{ x: 0, y: -1 },
            Direction::Down => state.pos + Point{ x: 0, y: 1 },
            Direction::Left => state.pos + Point{ x: -1, y: 0 },
            Direction::Right => state.pos + Point{ x: 1, y: 0 },
        };
        match map.get(&next) {
            Some(Tile::Empty) => {
                state.pos = next;
            },
            Some(Tile::Full) => {
                state.dir = match state.dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
            None => {
                return (seen, true);
            }
        }
    }
    unreachable!("Infinite loop");
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let (map, mut state) = parse(reader)?;
        let (seen,_) = find_path(&map, &state);

        println!("{}", repr(&map, &seen));
        Ok(seen.map.len())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (map, mut state) = parse(reader)?;
        let mut count :usize = 0;
        for y in map.min.y..=map.max.y {
            for x in map.min.y..=map.max.x {
                let point = (x, y);
                let mut map = map.clone();
                map.insert(point.into(), Tile::Full);
                let (_,offmap) = find_path(&map, &state);
                if !offmap {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
