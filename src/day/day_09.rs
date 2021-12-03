use aoc_core::{AoCDay, ErrorWrapper};
use aoc_core::parse;

pub struct Day09 {
    pub preamble_size: usize
}

type Num = u64;

impl Day09 {
    fn validate(&self, data: &Vec<Num>) -> Option<(usize, Num)> {
        let mut pointer = self.preamble_size;
        'outer: while pointer < data.len() {
            let p_start = pointer - self.preamble_size;
            let mut valid = false;
            'inner: for a in p_start..pointer {
                for b in a+1..pointer {
                    if data[a] + data[b] == data[pointer] {
                        valid = true;
                        break 'inner;
                    }
                }
            }
            if !valid {
                break 'outer;
            }
            pointer += 1;
        }
        if pointer < data.len() {
            Some((pointer, data[pointer]))
        } else {
            None
        }
    }
}

impl AoCDay for Day09 {
    fn day(&self) -> usize {
        09
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("26134589"), Some("3535124"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse::<Num>(input);
        Ok(self.validate(&data).expect("No weakness found").1.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse::<Num>(input);
        let invalid = self.validate(&data).expect("No weakness found").1;
        let mut weakness = 0;
        'outer: for a in 0..data.len()-1 {
            for b in a+1..data.len() {
                let segment = &data[a..b];
                if segment.iter().sum::<Num>() == invalid {
                    weakness = segment.iter().min().expect("min") + segment.iter().max().expect("max");
                    break 'outer;
                }
            }
        }
        Ok(weakness.to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day09 { preamble_size: 25 })
}

pub fn get_test_day(preamble_size: usize) -> Box<dyn AoCDay> {
    Box::new(Day09 { preamble_size })
}
