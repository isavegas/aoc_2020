use aoc_core::{bail, AoCDay, ErrorWrapper};
use bitflags::bitflags;
use std::collections::HashMap;

// TODO: Remove usage of bitflags? This feels a bit messy.
bitflags! {
    struct Response: u32 {
        const A = 0b00000000_00000000_00000000_00000001;
        const B = 0b00000000_00000000_00000000_00000010;
        const C = 0b00000000_00000000_00000000_00000100;
        const D = 0b00000000_00000000_00000000_00001000;
        const E = 0b00000000_00000000_00000000_00010000;
        const F = 0b00000000_00000000_00000000_00100000;
        const G = 0b00000000_00000000_00000000_01000000;
        const H = 0b00000000_00000000_00000000_10000000;
        const I = 0b00000000_00000000_00000001_00000000;
        const J = 0b00000000_00000000_00000010_00000000;
        const K = 0b00000000_00000000_00000100_00000000;
        const L = 0b00000000_00000000_00001000_00000000;
        const M = 0b00000000_00000000_00010000_00000000;
        const N = 0b00000000_00000000_00100000_00000000;
        const O = 0b00000000_00000000_01000000_00000000;
        const P = 0b00000000_00000000_10000000_00000000;
        const Q = 0b00000000_00000001_00000000_00000000;
        const R = 0b00000000_00000010_00000000_00000000;
        const S = 0b00000000_00000100_00000000_00000000;
        const T = 0b00000000_00001000_00000000_00000000;
        const U = 0b00000000_00010000_00000000_00000000;
        const V = 0b00000000_00100000_00000000_00000000;
        const W = 0b00000000_01000000_00000000_00000000;
        const X = 0b00000000_10000000_00000000_00000000;
        const Y = 0b00000001_00000000_00000000_00000000;
        const Z = 0b00000010_00000000_00000000_00000000;
    }
}

impl From<char> for Response {
    fn from(c: char) -> Self {
        match c {
            'a' => Response::A,
            'b' => Response::B,
            'c' => Response::C,
            'd' => Response::D,
            'e' => Response::E,
            'f' => Response::F,
            'g' => Response::G,
            'h' => Response::H,
            'i' => Response::I,
            'j' => Response::J,
            'k' => Response::K,
            'l' => Response::L,
            'm' => Response::M,
            'n' => Response::N,
            'o' => Response::O,
            'p' => Response::P,
            'q' => Response::Q,
            'r' => Response::R,
            's' => Response::S,
            't' => Response::T,
            'u' => Response::U,
            'v' => Response::V,
            'w' => Response::W,
            'x' => Response::X,
            'y' => Response::Y,
            'z' => Response::Z,
            _ => panic!("Unsupported key: {}", c),
        }
    }
}

fn parse<S>(input: S) -> Vec<Vec<Response>> where S: AsRef<str> {
    let mut groups = vec![];
    let mut cur_group = vec![];
    for line in input.as_ref().lines() {
        if !line.is_empty() {
            let mut response = Response::empty();
            for c in line.chars() {
                response |= Response::from(c);
            }
            cur_group.push(response);
        } else {
            let mut cg = vec![];
            std::mem::swap(&mut cur_group, &mut cg);
            groups.push(cg);
        }
    }
    if groups[groups.len() - 1] != cur_group {
        groups.push(cur_group);
    }
    groups
}

pub struct Day06;

impl AoCDay for Day06 {
    fn day(&self) -> usize {
        06
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("6534"), Some("3402"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse(input);
        let mut responses: HashMap<Response, usize> = HashMap::new();
        for group in &data {
            for c in "abcdefghijklmnopqrstuvwxyz".chars() {
                let r = Response::from(c);
                if group.iter().any(|response| *response & r != Response::empty()) {
                    *responses.entry(r).or_insert(0) += 1;
                }
            }
        }
        Ok(responses.values().sum::<usize>().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse(input);
        let mut responses: HashMap<Response, usize> = HashMap::new();
        for group in &data {
            for c in "abcdefghijklmnopqrstuvwxyz".chars() {
                let r = Response::from(c);
                if group.iter().all(|response| *response & r != Response::empty()) {
                    *responses.entry(r).or_insert(0) += 1;
                }
            }
        }
        Ok(responses.values().sum::<usize>().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day06)
}

