use aoc_2020::day::day_02::get_day;

const INPUT: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "2".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "1".to_string()
    );
}
