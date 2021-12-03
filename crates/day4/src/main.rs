const INPUT : &str = include_str!("input.txt");

fn main() {
    println!("{}", solve_problem1(INPUT));
    println!("{}", solve_problem2(INPUT));
}

fn solve_problem1(input : &str) -> i32 {
    0
}

fn solve_problem2(input : &str) -> i32 {
    0
}


#[allow(dead_code)]
const TEST_INPUT : &str = r#"
"#;

#[test]
fn test_problem1() {
    assert_eq!(solve_problem1(TEST_INPUT), 0);
}

#[test]
fn test_problem2() {
    assert_eq!(solve_problem1(TEST_INPUT), 0);
}