use aoc_2020::day::day_06::get_day;

const INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "11".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "6".to_string()
    );
}
