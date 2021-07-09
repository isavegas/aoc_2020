use aoc_core::{bail, AoCDay, ErrorWrapper};

use lazy_static::lazy_static;

pub struct Day1;

const INPUT: &'static str = include_str!("../input/day_01.txt");
type Num = u64;

lazy_static! {
    static ref EXPENSES: Vec<Num> = aoc_core::parse::numbers(INPUT);
}

fn valid2(a: &Num, b: &Num) -> Option<Num> {
    if a + b == 2020 {
        Some(a * b)
    } else {
        None
    }
}
fn valid3(a: &Num, b: &Num, c: &Num) -> Option<Num> {
    if a + b + c == 2020 {
        Some(a * b * c)
    } else {
        None
    }
}

impl AoCDay for Day1 {
    fn day(&self) -> usize {
        1
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("436404"), Some("274879808"))
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        let mut candidates = vec![];
        for (i, a) in EXPENSES.iter().enumerate() {
            if i < (EXPENSES.len() -  1) {
                for b in EXPENSES.iter().skip(i + 1) {
                    match valid2(a, b) {
                        Some(m) => candidates.push(m),
                        None => (),
                    }
                }
            }
        }
        match candidates.iter().max() {
            Some(m) => Ok(m.to_string()),
            None => bail!("Unable to find maximum!".to_string()),
        }
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        let mut candidates = vec![];
        for (i, a) in EXPENSES.iter().enumerate() {
            if i < (EXPENSES.len() -  2) {
                for b in EXPENSES.iter().skip(i + 1) {
                    for c in EXPENSES.iter().skip(i + 2) {
                        match valid3(a, b, c) {
                            Some(m) => candidates.push(m),
                            None => (),
                        }
                    }
                }
            }
        }
        match candidates.iter().max() {
            Some(m) => Ok(m.to_string()),
            None => bail!("Unable to find maximum!".to_string()),
        }
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day1)
}
