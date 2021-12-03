use aoc_core::{AoCDay, ErrorWrapper};
use aoc_core::parse;

pub struct Day10;

type Num = u64;

fn adapter_sort(mut adapters: Vec<Num>, target: &Num) -> Vec<Num> {
    adapters.sort();
    // No need to fiddle
    let max = adapters.iter().max().expect("max");
    if max > target {
        return adapters;
    }
    for (i, v) in adapters.iter().enumerate().rev() {
        println!("{} :: {}", i, v);
    }
    adapters
}

fn abs_difference(x: Num, y: Num) -> Num {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl AoCDay for Day10 {
    fn day(&self) -> usize {
        10
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut adapters = parse::<Num>(input);
        let target_joltage = adapters.iter().max().expect("max") + 3;
        println!("Joltage:         {}", target_joltage);
        println!("Adapters:        {:?}", adapters);
        adapters = adapter_sort(adapters, &target_joltage);
        let mut diff_1 = 0;
        let mut diff_3 = 0;
        let mut a = adapters[0];
        for i in 1..adapters.len()-1 {
            let b = adapters[i];
            match abs_difference(a, b) {
                1 => diff_1 += 1,
                3 => diff_3 += 1,
                _ => (),
            }
            if i == adapters.len() { break; }
            a = b
        }
        println!("Adapters Sorted: {:?}", adapters);
        Ok((diff_1 * diff_3).to_string())
    }
    fn part2(&self, _input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day10)
}
