use aoc_2020::day::day_09::get_test_day;

const INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_test_day(5)
            .part1(INPUT)
            .expect("Error"),
        "127".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_test_day(5)
            .part2(INPUT)
            .expect("Error"),
        "62".to_string()
    );
}
