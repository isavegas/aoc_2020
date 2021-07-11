use aoc_2020::day::day_03::get_day;

const INPUT: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "7".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "336".to_string()
    );
}
