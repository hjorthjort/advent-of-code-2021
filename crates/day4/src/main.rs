const INPUT : &str = include_str!("input.txt");

fn main() {
    println!("{}", solve_problem1(INPUT));
    println!("{}", solve_problem2(INPUT));
}

fn solve_problem1(input : &str) -> u32 {
    let mut input: Vec<&str> = input.trim().lines().filter(|l| l != &"").collect();
    let numbers: Vec<u8> = input.remove(0).split(",").map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<[[u8;5];5]> = Vec::new();
    for b in input.chunks(5) {
        let board: [[u8;5];5] = (*b).iter().map(|l| line_to_nums(l)).collect::<Vec<[u8;5]>>().try_into().unwrap();
        boards.push(board);
    }
    let mut drawn: Vec<u8> = Vec::new();
    for i in 0..numbers.len() {
        drawn.push(numbers[i]);
        for b in boards.iter() {
            if check_win(b, &drawn) {
                return
                    present_winner(b, &drawn);
            }
        }
    }
    0
}

fn present_winner(board: &[[u8;5];5], drawn: &Vec<u8>) -> u32 {
    println!("{:?}", board);
    println!("{:?}", drawn);
    let unmarked_sum : u32 = board.iter().flatten().filter(|x| !drawn.contains(*x)).map(|x| *x as u32).sum();
    let last = *drawn.last().unwrap();
    unmarked_sum * (last as u32)
}

fn line_wins(line: &[u8;5], drawn: &Vec<u8>) -> bool {
    for i in line {
        if !(drawn.contains(&i)) {
            return false; // this line is not a winner
        }
    }
    true
}

#[test]
fn test_line_wins() {
    assert_eq!(line_wins(&[3,4,2,1,9],  &Vec::from([1,2,3,4,5,6,7,8,9])), true);
    assert_eq!(line_wins(&[10,4,2,1,9], &Vec::from([1,2,3,4,5,6,7,8,9])), false);
}

fn transpose(mat: &[[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut out = [[0;5];5];
    for i in 0..5 {
        for j in 0..5 {
            out[i][j] = mat[j][i];
        }
    }
    out
}

#[test]
fn test_transpose() {
    assert_eq!(transpose(&[[1, 2, 3, 4, 5],[1, 2, 3, 4, 5],[1, 2, 3, 4, 5],[1, 2, 3, 4, 5],[1, 2, 3, 4, 5]]),
                 [[1, 1, 1, 1, 1], [2, 2, 2, 2, 2], [3, 3, 3, 3, 3], [4, 4, 4, 4, 4],  [5, 5, 5, 5, 5]]);
}

fn check_win(board : &[[u8;5];5], drawn: &Vec<u8>) -> bool {
    for l in board {
        if line_wins(l, drawn) {
            return true
        }
    }
    for l in transpose(board) {
        if line_wins(&l, drawn) {
            return true
        }
    }
    false
}

#[test]
fn test_check_win() {
    let board = [[14, 21, 17, 24, 4], [10, 16, 15, 9, 19], [18, 8, 23, 26, 20], [22, 11, 13, 6, 5], [2, 0, 12, 3, 7]];
    let drawn = Vec::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]);
    assert_eq!(check_win(&board, &drawn), true);
}

fn line_to_nums(line: &str) -> [u8;5] {
    line.split_ascii_whitespace().take(5).map(|x| x.parse().unwrap()).collect::<Vec<u8>>().try_into().unwrap()
}

#[test]
fn test_line_to_nums() {
    assert_eq!(line_to_nums("32 23 0 19 3"), [32, 23, 0, 19, 3]);
}

fn solve_problem2(input : &str) -> u32 {
    let mut input: Vec<&str> = input.trim().lines().filter(|l| l != &"").collect();
    let numbers: Vec<u8> = input.remove(0).split(",").map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<[[u8;5];5]> = Vec::new();
    for b in input.chunks(5) {
        let board: [[u8;5];5] = (*b).iter().map(|l| line_to_nums(l)).collect::<Vec<[u8;5]>>().try_into().unwrap();
        boards.push(board);
    }
    let mut drawn: Vec<u8> = Vec::new();
    let mut winners: Vec<[[u8;5];5]> = Vec::new();
    let mut win_draws : Vec<Vec<u8>> = Vec::new();
    for i in 0..numbers.len() {
        drawn.push(numbers[i]);
        for b in boards.iter() {
            if winners.contains(b) {
                continue;
            }
            if check_win(b, &drawn) {
                winners.push(*b);
                win_draws.push(drawn.clone());
            }
        }
    }
    present_winner(winners.last().unwrap(), win_draws.last().unwrap())
}


#[allow(dead_code)]
const TEST_INPUT : &str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

#[test]
fn test_problem1() {
    assert_eq!(solve_problem1(TEST_INPUT), 4512);
}

#[test]
fn test_problem2() {
    assert_eq!(solve_problem2(TEST_INPUT), 1924);
}