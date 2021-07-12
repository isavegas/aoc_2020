use aoc_2020::day::day_08::get_day;

const INPUT: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "5".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "8".to_string()
    );
}
