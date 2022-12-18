use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Stacks = Vec<VecDeque<char>>;

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read {}", file_path));

    let stacks = BufReader::new(file)
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold(vec![], |stacks, line| {
            if let Some(m) = parse_move(&line) {
                print_stack(&stacks);
                println!("{:?}", m);
                perform_move(stacks, m)
            } else {
                parse_stack_line(stacks, &line)
            }
        });

    println!("{}", get_tops(&stacks));
}

fn get_tops(stacks: &Stacks) -> String {
    stacks.iter().filter_map(|stack| stack.front()).collect()
}

fn parse_stack_line(stacks: Stacks, line: &str) -> Stacks {
    let mut stacks = stacks;

    let num_stacks = (line.chars().count() + 1) / 4;
    if stacks.len() < num_stacks {
        stacks.resize(num_stacks, VecDeque::new());
    }

    line.chars()
        .skip(1)
        .step_by(4)
        .enumerate()
        .for_each(|(i, c)| {
            if c.is_alphabetic() {
                stacks[i].push_back(c);
            }
        });

    stacks
}

fn print_stack(stacks: &Stacks) {
    println!();
    let height = stacks.iter().map(|stack| stack.len()).max().unwrap_or(0);
    for i in 0..height {
        for stack in stacks {
            let offset = height - stack.len();
            if i < offset {
                print!("    ");
            } else if let Some(c) = stack.get(i - offset) {
                print!("[{}] ", c);
            } else {
                print!("    ");
            }
        }
        println!();
    }
    for i in 0..stacks.len() {
        print!(" {}  ", i + 1);
    }
    println!();
}

fn parse_move(line: &str) -> Option<Move> {
    if !line.starts_with("move") {
        return None;
    }
    let mut parts = line.split_whitespace();
    let count = parts.nth(1)?.parse::<usize>().ok()?;
    let from = parts.nth(1)?.parse::<usize>().ok()? - 1;
    let to = parts.nth(1)?.parse::<usize>().ok()? - 1;
    Some(Move { count, from, to })
}

fn perform_move(stacks: Stacks, mv: Move) -> Stacks {
    let mut stacks = stacks;

    let keeping = stacks[mv.from].split_off(mv.count);

    for _ in 0..mv.count {
        if let Some(moving) = stacks[mv.from].pop_back() {
            stacks[mv.to].push_front(moving);
        }
    }

    stacks[mv.from] = keeping;

    stacks
}

mod test {
    use super::*;

    fn _test_parse_line(line: &str, initial_state: Stacks, expected_state: Stacks) {
        assert_eq!(parse_stack_line(initial_state, line), expected_state);
    }

    #[test]
    fn test_parse_line_1() {
        _test_parse_line("", Stacks::new(), vec![]);

        _test_parse_line(
            "[A] [B]",
            Stacks::new(),
            vec![VecDeque::from(['A']), VecDeque::from(['B'])],
        );

        _test_parse_line(
            "[A] [B] [C]",
            vec![
                VecDeque::from(['D']),
                VecDeque::new(),
                VecDeque::from(['E']),
            ],
            vec![
                VecDeque::from(['D', 'A']),
                VecDeque::from(['B']),
                VecDeque::from(['E', 'C']),
            ],
        )
    }

    #[test]
    fn test_parse_multiline() {
        let input = r#"[T] [V]                     [W]    
[V] [C] [P] [D]             [B]    
[J] [P] [R] [N] [B]         [Z]    
[W] [Q] [D] [M] [T]     [L] [T]    
[N] [J] [H] [B] [P] [T] [P] [L]    
[R] [D] [F] [P] [R] [P] [R] [S] [G]
[M] [W] [J] [R] [V] [B] [J] [C] [S]
[S] [B] [B] [F] [H] [C] [B] [N] [L]
 1   2   3   4   5   6   7   8   9 
"#;

        let stacks = input.lines().fold(vec![], parse_stack_line);

        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['T', 'V', 'J', 'W', 'N', 'R', 'M', 'S']),
                VecDeque::from(['V', 'C', 'P', 'Q', 'J', 'D', 'W', 'B']),
                VecDeque::from(['P', 'R', 'D', 'H', 'F', 'J', 'B']),
                VecDeque::from(['D', 'N', 'M', 'B', 'P', 'R', 'F']),
                VecDeque::from(['B', 'T', 'P', 'R', 'V', 'H']),
                VecDeque::from(['T', 'P', 'B', 'C']),
                VecDeque::from(['L', 'P', 'R', 'J', 'B']),
                VecDeque::from(['W', 'B', 'Z', 'T', 'L', 'S', 'C', 'N']),
                VecDeque::from(['G', 'S', 'L']),
            ]
        )
    }

    #[test]
    fn test_get_tops() {
        let mut stacks = vec![
            VecDeque::from(['A', 'B', 'C']),
            VecDeque::from(['D', 'E', 'F']),
            VecDeque::from(['G', 'H', 'I']),
        ];

        assert_eq!(get_tops(&stacks), "ADG");

        stacks[0].pop_front();
        assert_eq!(get_tops(&stacks), "BDG");

        stacks[1].pop_front();
        assert_eq!(get_tops(&stacks), "BEG");

        stacks[2].pop_front();
        assert_eq!(get_tops(&stacks), "BEH");
    }

    #[test]
    fn test_parse_move() {
        assert_eq!(
            parse_move("move 7 from 3 to 9"),
            Some(Move {
                count: 7,
                from: 2,
                to: 8
            })
        );

        assert_eq!(
            parse_move("move 14 from 4 to 7"),
            Some(Move {
                count: 14,
                from: 3,
                to: 6
            })
        );

        assert_eq!(parse_move(""), None);
        assert_eq!(parse_move(" 1   2   3   4   5   6   7   8   9 "), None);
        assert_eq!(parse_move("move 7 from 3 to "), None);
        assert_eq!(parse_move("move 7 from 3 to NaN"), None);
    }

    #[test]
    fn test_perform_move() {
        let stacks = vec![
            VecDeque::from(['A', 'B', 'C']),
            VecDeque::from(['D', 'E', 'F']),
            VecDeque::from(['G', 'H', 'I']),
        ];

        let stacks = perform_move(
            stacks,
            Move {
                count: 2,
                from: 0,
                to: 1,
            },
        );

        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['C']),
                VecDeque::from(['A', 'B', 'D', 'E', 'F']),
                VecDeque::from(['G', 'H', 'I']),
            ]
        );
    }

    #[test]
    fn test_multi_move() {
        let stacks = vec![
            VecDeque::from(['N', 'Z']),
            VecDeque::from(['D', 'C', 'M']),
            VecDeque::from(['P']),
        ];

        let stacks = perform_move(stacks, parse_move("move 1 from 2 to 1").unwrap());

        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['D', 'N', 'Z']),
                VecDeque::from(['C', 'M']),
                VecDeque::from(['P']),
            ]
        );

        let stacks = perform_move(stacks, parse_move("move 3 from 1 to 3").unwrap());

        assert_eq!(
            stacks,
            vec![
                VecDeque::from([]),
                VecDeque::from(['C', 'M']),
                VecDeque::from(['D', 'N', 'Z', 'P']),
            ]
        );

        let stacks = perform_move(stacks, parse_move("move 2 from 2 to 1").unwrap());

        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['C', 'M']),
                VecDeque::from([]),
                VecDeque::from(['D', 'N', 'Z', 'P']),
            ]
        );

        let stacks = perform_move(stacks, parse_move("move 1 from 1 to 2").unwrap());

        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['M']),
                VecDeque::from(['C']),
                VecDeque::from(['D', 'N', 'Z', 'P']),
            ]
        );
    }
}
