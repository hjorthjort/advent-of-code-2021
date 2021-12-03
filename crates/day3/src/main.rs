const INPUT : &str = include_str!("input.txt");

fn main() {
    println!("{}", solve_problem1(INPUT));
    println!("{}", solve_problem2(INPUT));
}

fn solve_problem1(inp : &str) -> i32 {
    let ls : Vec<String> = inp.trim().lines().map(String::from).collect();
    let (counts, len) = count_1s(&ls);
    let gamma : String = counts.iter().map(|x| { if *x < 0 {"0"} else {"1"}}).collect();
    let gamma : i32 = parse_binary(&gamma);
    let epsilon : i32 = 2i32.pow(len as u32) - 1 - gamma;
    gamma * epsilon
}

fn solve_problem2(inp : &str) -> i32 {
    let mut ls_oxygen : Vec<String> = inp.trim().lines().map(String::from).collect();
    let mut ls_co2 = ls_oxygen.clone();
    let (_, len) = count_1s(&ls_oxygen);
    for i in 0..len {
        let (counts, _) = count_1s(&ls_oxygen);
        ls_oxygen = filter_lines_common(counts, &ls_oxygen, i);
        if ls_oxygen.len() == 1 { break; }
    }
    for i in 0..len {
        let (counts, _) = count_1s(&ls_co2);
        ls_co2 = filter_lines_uncommon(counts, &ls_co2, i);
        if ls_co2.len() == 1 { break; }
    }
    let oxy = parse_binary(&ls_oxygen[0]);
    let co2 = parse_binary(&ls_co2[0]);
    oxy*co2
}

fn filter_lines_uncommon(counts : Vec<i32>, lines : &Vec<String>, pos: usize) -> Vec<String> {
    let mut res = Vec::new();
    for line in lines {
        if (counts[pos] < 0) != (line.chars().nth(pos).unwrap() == '0') {
            res.push(String::clone(line));
        }
    }
    res
}

fn filter_lines_common(counts : Vec<i32>, lines : &Vec<String>, pos: usize) -> Vec<String> {
    let mut res = Vec::new();
    for line in lines {
        if (counts[pos] < 0) == (line.chars().nth(pos).unwrap() == '0') {
            res.push(String::clone(line));
        }
    }
    res
}

// Returns a vector which counts the 1s in each position, subtracted by the number of non-1s in each position.
fn count_1s(ls : &Vec<String>) -> (Vec<i32>, usize) {
    let len = ls[0].len();
    let mut counts : Vec<i32> = Vec::new();
    counts.resize(len as usize, 0);
    let mut neg = Vec::new();
    neg.resize(len, -1);
    let neg = neg; // Immutable
    for l in ls {
        let line = line_to_vec(l);
        let line = vec_add(&line, &line);
        let line = vec_add(&line, &neg);
        counts = vec_add(&counts, &line);
    }
    (counts, len)
}

fn line_to_vec(line : &str) -> Vec<i32> {
    let v = line.trim().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    v
}

fn vec_add(a : &Vec<i32>, b : &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..a.len() {
        res.push(a[i] + b[i]);
    }
    res
}

fn parse_binary(bin_str : &String) -> i32 {
    i32::from_str_radix(bin_str.as_str(), 2).unwrap()
}

#[test]
fn test_line_to_vec() {
    assert_eq!(line_to_vec("010101"), Vec::from([0,1,0,1,0,1]));
}

#[test]
fn test_vec_add() {
    assert_eq!(vec_add(&Vec::from([0,1,2,3]), &Vec::from([0,-1,-2,-3])), Vec::from([0,0,0,0]));
}

#[test]
fn test_parse_binary() {
    assert_eq!(parse_binary(&String::from("10110")), 22);
    assert_eq!(parse_binary(&String::from("01001")), 9);
    assert_eq!(parse_binary(&String::from("00000")), 0);
    assert_eq!(parse_binary(&String::from("11111")), 2i32.pow(5) - 1);
}

const TEST_INPUT : &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

#[test]
fn problem1() {
    assert_eq!(solve_problem1(TEST_INPUT), 198);
}

#[test]
fn problem2() {
    assert_eq!(solve_problem2(TEST_INPUT), 230);
}