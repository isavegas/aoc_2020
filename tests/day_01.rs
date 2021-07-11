use aoc_2020::day::day_01::get_day;

const INPUT: &str = r#"1721
979
366
299
675
1456"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "514579".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "241861950".to_string()
    );
}
