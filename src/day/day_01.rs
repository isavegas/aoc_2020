use aoc_core::{bail, AoCDay, ErrorWrapper};

pub struct Day01;

type Num = u64;

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

impl AoCDay for Day01 {
    fn day(&self) -> usize {
        1
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("436404"), Some("274879808"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let expenses = aoc_core::parse::<Num>(input);
        let mut candidates = vec![];
        for (i, a) in expenses.iter().enumerate() {
            if i < (expenses.len() -  1) {
                for b in expenses.iter().skip(i + 1) {
                    if let Some(m) = valid2(a, b) { candidates.push(m) }
                }
            }
        }
        match candidates.iter().max() {
            Some(m) => Ok(m.to_string()),
            None => bail!("Unable to find maximum!".to_string()),
        }
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let expenses = aoc_core::parse::<Num>(input);
        let mut candidates = vec![];
        for (i, a) in aoc_core::parse::<Num>(input).iter().enumerate() {
            if i < (expenses.len() -  2) {
                for b in expenses.iter().skip(i + 1) {
                    for c in expenses.iter().skip(i + 2) {
                        if let Some(m) = valid3(a, b, c) { candidates.push(m) }
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
    Box::new(Day01)
}
