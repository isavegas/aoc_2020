use aoc_core::{AoCDay, ErrorWrapper, Vec2};

use lazy_static::lazy_static;

pub struct Day03;

const INPUT: &'static str = include_str!("../input/day_03.txt");

lazy_static! {
    static ref TREE_MAP: TreeMap = parse_map(INPUT);
}

fn parse_map(input: &str) -> TreeMap {
    let mut map = TreeMap::default();
    for line in input.lines() {
        map.data.push(line.chars().map(|v| match v {
            '#' => Entity::Tree,
            '.' => Entity::Open,
            _ => panic!("Broken input!"),
        }).collect());
    }
    map
}

#[derive(Debug, Clone)]
enum Entity {
    Open,
    Tree,
}

#[derive(Debug, Clone, Default)]
struct TreeMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<Entity>>
}

fn tree_count(slope: &Vec2<usize>) -> usize {
    let mut location = Vec2::default();
    let mut trees = 0;
    while location.y < TREE_MAP.height() {
        match TREE_MAP.get(location).expect("Out of bounds") {
            Entity::Open => (),
            Entity::Tree => trees += 1,
        }
        location.x += slope.x;
        location.y += slope.y;
    }
    trees
}

impl TreeMap {
    fn get(&self, coord: Vec2<usize>) -> Option<&Entity> {
        let row = self.data.get(coord.y)?;
        row.get(coord.x % row.len())
    }
    fn height(&self) -> usize {
        self.data.len()
    }
}

impl AoCDay for Day03 {
    fn day(&self) -> usize {
        3
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("176"), Some("5872458240"))
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        Ok(tree_count(&Vec2::new(3, 1)).to_string())
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let slopes = vec![
            Vec2::new(1, 1),
            Vec2::new(3, 1),
            Vec2::new(5, 1),
            Vec2::new(7, 1),
            Vec2::new(1, 2),
        ];
        Ok(slopes.iter()
            .map(tree_count)
            .fold(1, |m, v| m * v)
            .to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day03)
}
