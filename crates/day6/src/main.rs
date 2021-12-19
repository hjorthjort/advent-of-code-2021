const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", solve_problem1(INPUT));
    println!("{}", solve_problem2(INPUT));
}

fn solve_problem1(input: &str) -> u64 {
    solve_problem(input, 80)
}

fn solve_problem2(input: &str) -> u64 {
    solve_problem(input, 256)
}

fn solve_problem(input: &str, days: usize) -> u64 {
    let mut spawn_nums: Vec<usize> = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    spawn_nums.sort();
    let mut spawn: [u64;9] = [0;9];
    for n in spawn_nums {
        spawn[n] += 1
    }
    for _ in 0..days {
        spawn = shift(spawn);
    }
    spawn.iter().sum()
}

fn shift(spawn: [u64;9]) -> [u64;9] {
    let mut res = [0;9];
    for i in 0..8 {
        res[i] = spawn[i+1];
    }
    res[8] = spawn[0];
    res[6] += spawn[0];
    res
}

#[test]
fn test_shift() {
    assert_eq!(shift([1,0,3,4,4,5,6,8,2]), [0, 3, 4, 4, 5, 6, 9, 2, 1]);
}

#[allow(dead_code)]
const TEST_INPUT: &str = "3,4,3,1,2";

#[test]
fn test_problem1() {
    assert_eq!(solve_problem1(TEST_INPUT), 5934);
}

#[test]
fn test_problem2() {
    assert_eq!(solve_problem2(TEST_INPUT), 26984457539);
}
