const INPUT: &str = include_str!("input.txt");

use std::collections::HashMap;
use std::cmp::{max,min};

fn main() {
    println!("{}", solve_problem1(INPUT));
    println!("{}", solve_problem2(INPUT));
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut res = Vec::new();
    for l in input.trim().lines() {
        let mut split = l.split(" -> ").map(|x| x.split(",")).flatten();
        let coord = (
            split.next().unwrap().trim().parse().unwrap(),
            split.next().unwrap().trim().parse().unwrap(),
            split.next().unwrap().trim().parse().unwrap(),
            split.next().unwrap().trim().parse().unwrap(),
        );
        res.push(coord);
    }
    res
}

fn solve_problem1(input: &str) -> usize {
    let coords: Vec<(i32, i32, i32, i32)> = parse_input(input);
    let coords: Vec<(i32, i32, i32, i32)> = coords.iter().filter(|x| filter_hor_ver(x)).map(|x| *x).collect();
    let mut covered: HashMap<(i32, i32), i32> = HashMap::new(); // Coord + count
    for (sx, sy, ex, ey) in coords {
        let (sx, ex) = (min(sx,ex), max(sx, ex));
        let (sy, ey) = (min(sy,ey), max(sy, ey));
        for i in sx..(ex+1) {
            for j in sy..(ey+1) {
                let c = (i, j);
                match covered.get(&c) {
                    None => covered.insert(c, 1),
                    Some(v) => covered.insert(c, v + 1)
                };
            }
        }
    }
    let overlaps = covered.values().filter(|x| **x > 1).count();
    overlaps
}

fn filter_hor_ver(c: &(i32, i32, i32, i32)) -> bool {
    c.0 == c.2 || c.1 == c.3
}

fn solve_problem2(input: &str) -> usize {
    let paths: Vec<(i32, i32, i32, i32)> = parse_input(input);
    //let paths: Vec<(i32, i32, i32, i32)> = paths.iter().filter(|x| filter_hor_ver(x)).map(|x| *x).collect();
    let mut covered: HashMap<(i32, i32), i32> = HashMap::new(); // Coord + count
    for path in paths {
        let (sx, sy, ex, ey) = path;
        let (sx, ex) = (min(sx,ex), max(sx, ex));
        let (sy, ey) = (min(sy,ey), max(sy, ey));
        for i in sx..(ex+1) {
            for j in sy..(ey+1) {
                let c = (i, j);
                if sx == ex || sy == ey || on_path(c, path) {
                    match covered.get(&c) {
                        None => covered.insert(c, 1),
                        Some(v) => covered.insert(c, v + 1)
                    };
                }
            }
        }
    }
    let overlaps = covered.values().filter(|x| **x > 1).count();
    overlaps
}

fn on_path(point : (i32, i32), path: (i32, i32, i32, i32)) -> bool {
    let dx = (point.0 - path.0).abs();
    let dy = (point.1 - path.1).abs();
    dx == dy
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

#[test]
fn test_problem1() {
    assert_eq!(solve_problem1(TEST_INPUT), 5);
}

#[test]
fn test_problem2() {
    assert_eq!(solve_problem2(TEST_INPUT), 12);
}
