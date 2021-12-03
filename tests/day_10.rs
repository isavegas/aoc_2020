use aoc_2020::day::day_10::get_day;

const INPUT_1: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

const INPUT_2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;


#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT_1)
            .expect("Error"),
        "".to_string()
    );
}

#[test]
pub fn part1_2() {
    assert_eq!(
        get_day()
            .part1(INPUT_2)
            .expect("Error"),
        "".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT_1)
            .expect("Error"),
        "".to_string()
    );
}
