const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", solve_problem1(INPUT));
    println!("{}", solve_problem2(INPUT));
}

fn solve_problem1(input : &str) -> i32 {
    let input : Vec<Command> = input.trim().lines().map(line_to_command).collect();
    let mut h : i32 = 0;
    let mut v : i32 = 0;
    for Command { direction, steps } in input.iter() {
        match direction.as_str() {
            "up" => h -= steps,
            "down" => h += steps,
            "forward" => v += steps,
            "backward" => v -= steps,
            _ => println!("no")
        }
    }
    h * v
}

fn solve_problem2(input : &str) -> i32 {
    let input : Vec<Command> = input.trim().lines().map(line_to_command).collect();
    let mut a : i32 = 0;
    let mut h : i32 = 0;
    let mut v : i32 = 0;
    for Command { direction, steps } in input.iter() {
        match direction.as_str() {
            "up" => a -= steps,
            "down" => a += steps,
            "forward" => { v += steps; h += steps * a },
            "backward" => { v -= steps; h += steps * a },
            _ => println!("no")
        }
    }
    h * v
}


#[derive(Debug, PartialEq)]
struct Command {direction : String, steps : i32}

fn line_to_command(line : &str) -> Command {
    let mut line = line.split_ascii_whitespace();
    let d : &str = line.next().unwrap().into();
    let s : i32 = line.next().unwrap().parse().unwrap();
    Command{ direction: String::from(d), steps : s }
}


const TEST_INPUT : &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

#[test]
fn line_parser() {
    let res = Command { direction: String::from("down"), steps: 20 };
    assert_eq!(res, line_to_command("down 20"));
}

#[test]
fn problem1() {
    assert_eq!(solve_problem1(TEST_INPUT), 150);
}

#[test]
fn problem2() {
    assert_eq!(solve_problem2(TEST_INPUT), 900);
}