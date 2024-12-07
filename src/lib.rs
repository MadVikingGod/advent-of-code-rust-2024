use core::hash::Hash;
use std::collections::HashMap;
use std::fmt::Display;
use anyhow::anyhow;
use anyhow::Result;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> where T: Hash + Copy{
    pub x: T,
    pub y: T,
}
impl<T> From<(T,T)> for Point<T>
where T: Hash + Copy + Ord
{
    fn from(p: (T,T)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}
impl<T> Point<T>
where
    T: Hash + Copy + Ord,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    pub fn max(&self, other: &Self) -> Self {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
    pub fn min(&self, other: &Self) -> Self {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}
impl<T> std::ops::Add for Point<T>
where
    T: Hash + Copy + Ord + std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl<T> Display for Point<T> where T: Display + Hash + Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Map<Tile> where  {
    map: HashMap<Point<i32>, Tile>,
    pub min: Point<i32>,
    pub max: Point<i32>,
}

impl<Tile> Map<Tile>
{
    pub fn new() -> Self {
        Map {
            map: HashMap::new(),
            min: (0,0).into(),
            max: (0,0).into(),
        }
    }
    pub fn get(&self, point: &Point<i32>) -> Option<&Tile> {
        self.map.get(point)
    }

    pub fn insert(&mut self, point: Point<i32>, tile: Tile) {
        self.max = self.max.max(&point);
        self.min = self.min.min(&point);
        self.map.insert(point, tile);
    }
    pub fn parse(reader: impl std::io::BufRead) -> Result<Self>
    where
    Tile: TryFrom<char>{
        let (map,_) = Self::parse_with_start(reader, &'\u{00FF}')?;
        Ok(map)
    }
    pub fn parse_with_start(reader: impl std::io::BufRead, start: &char) -> Result<(Self,Point<i32>)>
    where Tile: TryFrom<char>{
        let mut map: Map<Tile> = Self::new();
        let mut start_point = (0,0).into();
        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line?.chars().enumerate() {
                let point = (x as i32, y as i32);
                if c == *start {
                    start_point = point.into();
                }
                let tile: Tile = match c.try_into()
                {
                    Ok(tile) => tile,
                    Err(_) => return Err(anyhow!("Invalid character: {}", c))
                };
                map.insert(point.into(), tile);
            }
        }
        Ok((map, start_point))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
