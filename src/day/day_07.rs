use aoc_core::{AoCDay, ErrorWrapper};
use std::collections::HashMap;

type Num = u64;

// TODO: Better parsing code than this. It's horribly messy, but I want to focus on progress atm.
fn parse(input: &str) -> (HashMap<String, Num>, HashMap<Num, HashMap<Num, Num>>) {
    let mut registry: HashMap<String, Num> = HashMap::new();
    let mut map: HashMap<Num, HashMap<Num, Num>> = HashMap::new();
    let mut id_count = 0;
    registry.insert("shiny gold".to_string(), 0);
    for line in input.lines() {
        let mut parts = line.split("bags contain ");
        let bag_id = *registry.entry(parts.next().expect("No parts").trim().to_string()).or_insert({id_count += 1; id_count});
        let bag = map.entry(bag_id).or_insert(HashMap::new());
        let items = parts.next().expect("No items").trim();
        if items != "no other bags." {
            for i in items.split(", ") {
                let mut f = i.split(' ');
                let count = f.next().expect("Integer").parse::<Num>().expect("Integer Parseable");
                let bag_color = format!("{} {}", f.next().expect("color_1"), f.next().expect("color_2"));
                let int_bag_id = *registry.entry(bag_color).or_insert({id_count += 1; id_count});
                bag.insert(int_bag_id, count);
            }
        }
    }
    (registry, map)
}

fn contains_gold(map: &HashMap<Num, HashMap<Num, Num>>, id: Num) -> bool {
    map.get(&id).expect("Missing id").iter().any(|(k, _v)| *k == 0 || contains_gold(map, *k))
}

fn calc_quantity(map: &HashMap<Num, HashMap<Num, Num>>, values: &mut HashMap<Num, Num>, id: Num) -> Num {
    match values.get(&id) {
        Some(v) => *v,
        None => {
            match map.get(&id) {
                Some(bags) => {
                    let mut v = 0;
                    for (b, q) in bags.iter() {
                        v += q + (q * calc_quantity(map, values, *b));
                    }
                    values.insert(id, v);
                    v
                },
                None => 0,
            }
        }
    }
}

pub struct Day07;

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        07
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("265"), Some("14177"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (registry, map) = parse(input);
        Ok(registry.values().filter(|k| **k != 0 && contains_gold(&map, **k)).count().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut values: HashMap<Num, Num> = HashMap::new();
        let (_registry, map) = parse(input);
        Ok(calc_quantity(&map, &mut values, 0).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
