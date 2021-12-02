const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn solve_problem1(input : &str) -> usize {
    solve_problem1_n(numbers(input))
}

fn solve_problem2(input : &str) -> usize {
    solve_problem2_n(numbers(input))
}

#[allow(unused)]
fn solve_problem1_n(ns : Vec<i32>) -> usize {
    let mut count = 0;
    let mut prev : &i32 = &ns[0];
    for x in ns.iter() {
        if (prev < x) {
            count += 1;
        }
        prev = x;
    }
    return count;
}

#[allow(unused)]
fn solve_problem2_n(ns : Vec<i32>) -> usize {
    solve_problem1_n(ns
        .windows(3)
        .map(|x| -> i32 {x.iter().sum() })
        .collect::<Vec<i32>>()
    )
}

fn numbers(input: &str) -> Vec<i32> {
    let numbers = input
        .trim()
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    numbers
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};

    #[test]
    fn problem1() {
        let input =  r#"
199
200
208
210
200
207
240
269
260
263
"#;
        let expected = 7;
        let actual = solve_problem1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;
        let expected = 5;
        let actual = solve_problem2(input);
        assert_eq!(expected, actual);
    }
}