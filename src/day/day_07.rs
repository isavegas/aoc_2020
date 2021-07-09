use aoc_core::{AoCDay, ErrorWrapper};

use lazy_static::lazy_static;

pub struct Day07;

type Num = u64;
const INPUT: &str = include_str!("../input/day_07.txt");

lazy_static! {
    static ref DATA: Vec<Num> = aoc_core::parse::numbers(INPUT);
}

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        7
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
