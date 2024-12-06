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

type Point = (i32, i32);
#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone)]
enum Tile {
    Off,
    Full,
    Empty,
}

#[derive(Clone)]
struct Map {
    map: HashMap<Point, Tile>,
    bounds: Point,
}

#[derive(Clone)]
struct State {
    pos: Point,
    dir: Direction,
}

impl Map {
    fn parse<R: BufRead>(reader: R) -> Result<(Self, State)> {
        let mut map = HashMap::new();
        let mut state = State {
            pos: (0, 0),
            dir: Direction::Up,
        };
        let lines = reader.lines();
        let mut y_max = 0;
        let mut x_max = 0;
        for (y, line) in lines.enumerate() {
            y_max = y as i32;
            let line = line?;
            x_max = line.len() as i32;
            for (x, c) in line.chars().enumerate() {
                let point = (x as i32, y as i32);
                let tile = match c {
                    '#' => Tile::Full,
                    '.' => Tile::Empty,
                    '^' => {
                        state.pos = point;
                        Tile::Empty
                    },
                    _ => return Err(anyhow!("Invalid character: {}", c))
                };
                map.insert(point, tile);
            }
        }
        Ok((Map{map:map, bounds:(x_max, y_max)}, state))
    }
    fn get(&self, point: &Point) -> Tile {
        self.map.get(point).cloned().unwrap_or(Tile::Off)
    }
    fn repr(&self, seen:&Seen) -> String {
        let mut result = String::new();
        for y in 0..self.bounds.1 {
            for x in 0..self.bounds.0 {
                let point = (x, y);
                let c = match (self.get(&point), seen.map.contains_key(&point)) {
                    (Tile::Full,_) => '#',
                    (Tile::Empty, false) => '.',
                    (Tile::Empty, true) => 'X',
                    (Tile::Off,_) => {continue},
                };
                result.push(c);
            }
            result.push('\n');
        }
        result
    }
}
struct Seen {
    map: HashMap<Point, HashSet<Direction>>
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

fn find_path(map: &Map, start: &State) -> (Seen, bool) {
    let mut seen = Seen::new();
    let mut state = start.clone();
    loop {
        if seen.contains(&state) {
            return (seen, false);
        }
        seen.insert(&state);
        let next = match state.dir {
            Direction::Up => (state.pos.0, state.pos.1 - 1),
            Direction::Down => (state.pos.0, state.pos.1 + 1),
            Direction::Left => (state.pos.0 - 1, state.pos.1),
            Direction::Right => (state.pos.0 + 1, state.pos.1),
        };
        match map.get(&next) {
            Tile::Empty => {
                state.pos = next;
            },
            Tile::Full => {
                state.dir = match state.dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
            Tile::Off => {
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

        let (map, mut state) = Map::parse(reader)?;

        let mut seen = Seen::new();
        let (seen,_) = find_path(&map, &state);

        println!("{}", map.repr(&seen));
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
        let (map, mut state) = Map::parse(reader)?;
        let mut count :usize = 0;
        for y in 0..=map.bounds.1 {
            for x in 0..=map.bounds.0 {
                let point = (x, y);
                let mut map = map.clone();
                map.map.insert(point, Tile::Full);
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
