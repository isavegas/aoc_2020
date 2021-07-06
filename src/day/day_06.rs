use crate::*;

use lazy_static::lazy_static;

pub struct Day06;

const INPUT: &'static str = include_str!("../input/day_06.txt");

lazy_static! {
    static ref DATA: Vec<Vec<bool>> = parse(INPUT);
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    for s in input.split("\n\n") {
        println!("{}", s.len());
    }
    vec![]
}


impl AoCDay for Day06 {
    fn day(&self) -> usize {
        06
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
    Box::new(Day06)
}
