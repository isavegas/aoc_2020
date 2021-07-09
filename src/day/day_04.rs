use aoc_core::{AoCDay, ErrorWrapper};

use lazy_static::lazy_static;

pub struct Day04;

const INPUT: &str = include_str!("../input/day_04.txt");

lazy_static! {
    static ref DATA: Vec<Passport> = parse(INPUT);
}

fn parse(input: &str) -> Vec<Passport> {
    let mut database = vec![];
    let mut passport = Passport::default();
    for line in input.trim().lines().map(str::trim) {
        if line.is_empty() {
            // Newline. Insert the last passport
            let mut insert_passport = Passport::default();
            std::mem::swap(&mut passport, &mut insert_passport);
            database.push(insert_passport);
        } else {
            for pair in line.split_whitespace() {
                passport.add_entry(pair);
            }
        }
    }
    database.push(passport);
    database
}

#[derive(Debug, Clone, Default)]
struct Passport {
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub eyr: Option<String>,
    pub hcl: Option<String>,
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub hgt: Option<String>,
    pub cid: Option<String>,
}

impl Passport {
    fn add_entry(&mut self, entry: &str) {
        match entry.split_once(':') {
            Some((id, value)) => match id {
                "ecl" => self.ecl = Some(value.to_string()),
                "pid" => self.pid = Some(value.to_string()),
                "eyr" => self.eyr = Some(value.to_string()),
                "hcl" => self.hcl = Some(value.to_string()),
                "byr" => self.byr = Some(value.to_string()),
                "iyr" => self.iyr = Some(value.to_string()),
                "hgt" => self.hgt = Some(value.to_string()),
                "cid" => self.cid = Some(value.to_string()),
                _ => panic!("Bad input"),
            },
            _ => panic!("Bad input"),
        }
    }
    fn missing(&self) -> bool {
        self.ecl.is_none()
            || self.pid.is_none()
            || self.eyr.is_none()
            || self.hcl.is_none()
            || self.byr.is_none()
            || self.iyr.is_none()
            || self.hgt.is_none()
        //|| self.cid.is_none() // allowed
    }
    fn valid(&self) -> bool {
        // Checking self.missing() allows us to blindly use .unwrap(),
        // leaving aside the special case of `cid`
        if self.missing() {
            return false;
        }

        {
            let byr = self
                .byr
                .as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0);
            if !(1920..=2002).contains(&byr) {
                return false;
            }
        }

        {
            let iyr = self
                .iyr
                .as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0);
            if !(2010..=2020).contains(&iyr) {
                return false;
            }
        }

        {
            let eyr = self
                .eyr
                .as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0);
            if !(2020..=2030).contains(&eyr) {
                return false;
            }
        }

        {
            let hgt_v = self.hgt.as_ref().unwrap();
            if hgt_v.len() < 3 {
                return false;
            }
            let hgt = hgt_v[..hgt_v.len() - 2]
                .parse::<usize>()
                .unwrap_or(0);
            if hgt_v.ends_with("cm") {
                if !(150..=193).contains(&hgt) {
                    return false;
                }
            } else if hgt_v.ends_with("in") {
                if !(59..=76).contains(&hgt) {
                    return false;
                }
            } else {
                return false;
            }
        }

        {
            let hcl_v = self.hcl.as_ref().unwrap();
            if hcl_v.len() != 7
                || !hcl_v.starts_with('#')
                || !hcl_v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
            {
                return false;
            }
        }

        {
            if !matches!(self.ecl.as_ref().unwrap().as_str(), "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
                return false
            }
        }

        {
            let pid_v = self.pid.as_ref().unwrap();
            if pid_v.len() != 9 || !pid_v.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }

        true
    }
}

impl AoCDay for Day04 {
    fn day(&self) -> usize {
        4
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("235"), Some("194"))
    }
    fn part1(&self) -> Result<String, ErrorWrapper> {
        Ok(DATA.iter().filter(|p| !p.missing()).count().to_string())
    }
    fn part2(&self) -> Result<String, ErrorWrapper> {
        Ok(DATA.iter().filter(|p| p.valid()).count().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day04)
}
