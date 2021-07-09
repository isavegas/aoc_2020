use aoc_core::{AoCDay, ErrorWrapper};

use lazy_static::lazy_static;

pub struct Day02;

type Num = usize;
const INPUT: &'static str = include_str!("../input/day_02.txt");

lazy_static! {
    static ref DATABASE: Vec<Entry> = parse(INPUT);
}

#[derive(Clone,Debug)]
struct Entry {
    pub required_character: char,
    pub required_min: Num,
    pub required_max: Num,
    pub password: String,
}

fn parse_line(line: &str) -> Option<Entry> {
    match line.find(':') {
        Some(c) => {
            let (pre, post) = line.split_at(c);
            let pass = post[1..].trim();
            match pre.find(' ') {
                Some(s) => {
                    let (req_range, req_post) = pre.split_at(s);
                    if req_post.len() != 2 { return None; }
                    let req_char = req_post.chars().nth(1);
                    match req_range.find('-') {
                        Some(h) => {
                            let (req_min, req_min_post) = req_range.split_at(h);
                            let min = req_min.parse::<Num>();
                            let max = req_min_post[1..].parse::<Num>();
                            if req_char.is_some() && min.is_ok() && max.is_ok() {
                                Some(Entry {
                                    required_character: req_char.unwrap(),
                                    required_min: min.unwrap(),
                                    required_max: max.unwrap(),
                                    password: pass.to_string(),
                                })
                            } else {
                                println!("Error parsing range: {}", line);
                                None
                            }
                        },
                        None => { println!("Error parsing hyphen: {}", line); None},
                    }
                },
                None => { println!("Error parsing space: {}", line); None},
            }
        },
        None => { println!("Error parsing colon: {}", line); None},
    }
}

fn parse(input: &str) -> Vec<Entry> {
    input.lines()
        .map(str::trim)
        .filter(|f| f.len() > 0)
        .map(parse_line)
        .map(Option::unwrap)
        .collect()
}

fn is_valid1(entry: &&Entry) -> bool {
    let req_count = entry.password.chars().filter(|c| *c == entry.required_character).count() as Num;
    req_count >= entry.required_min && req_count <= entry.required_max
}

fn is_valid2(entry: &&Entry) -> bool {
    let first = entry.password.chars().nth(entry.required_min - 1).unwrap() == entry.required_character;
    let second = entry.password.chars().nth(entry.required_max - 1).unwrap() == entry.required_character;
    (first && !second) || (!first && second)
}

impl AoCDay for Day02 {
    fn day(&self) -> usize {
        2
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("474"), Some("745"))
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        //println!("{:?}", *DATABASE);
        Ok(DATABASE.iter().filter(is_valid1).count().to_string())
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        Ok(DATABASE.iter().filter(is_valid2).count().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day02)
}
