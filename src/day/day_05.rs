use aoc_core::{AoCDay, ErrorWrapper};

pub struct Day05;

fn bpart_index_flatten(bpart_index: &[bool]) -> usize {
    bpart_index.iter().rev().enumerate().map(|(i, b)| match b {
        false => 2_usize.pow(i as u32),
        true => 0,
    }).sum()
}

#[derive(Clone, Debug, Default)]
struct Seat {
    x: usize,
    y: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.y * 8 + self.x
    }
    fn parse(encoded: &str) -> Seat {
        let row = bpart_index_flatten(&encoded.chars().take(7).map(|c| match c {
            'F' => true,
            'B' => false,
            _ => panic!(),
        }).collect::<Vec<_>>());
        let column = bpart_index_flatten(&encoded.chars().skip(7).map(|c| match c {
            'L' => true,
            'R' => false,
            _ => panic!(),
        }).collect::<Vec<_>>());
        Seat { y: row, x: column }
    }
}

impl AoCDay for Day05 {
    fn day(&self) -> usize {
        5
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("861"), Some("633"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        Ok(input.lines().map(Seat::parse).map(|s| s.id()).max().unwrap().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        // true == empty. Simplifies logic in iter chain.
        let mut flat = vec![true; 128 * 8];
        for id in input.lines().map(Seat::parse).map(|s| s.id()) {
            flat[id] = false;
        }
        Ok(flat.iter().enumerate().skip_while(|(_, v)| **v).find(|(_, v)| **v).unwrap().0.to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day05)
}
