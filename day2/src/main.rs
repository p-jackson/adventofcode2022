use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file_path = "input.txt";
    let f = File::open(file_path)
        .expect(format!("Should have been able to read {}", file_path).as_str());
    let reader = BufReader::new(f);

    let total_score = reader
        .lines()
        .filter_map(|line_result| {
            line_result
                .map(|line| get_choice_score(&line) + get_match_score(&line))
                .ok()
        })
        .sum::<u32>();

    println!("{}", total_score);
}

fn get_choice_score(line: &str) -> u32 {
    if line.contains("X") {
        1
    } else if line.contains("Y") {
        2
    } else if line.contains("Z") {
        3
    } else {
        0
    }
}

fn get_match_score(line: &str) -> u32 {
    if (line.contains("A") && line.contains("X"))
        || (line.contains("B") && line.contains("Y"))
        || (line.contains("C") && line.contains("Z"))
    {
        3
    } else if (line.contains("A") && line.contains("Y"))
        || (line.contains("B") && line.contains("Z"))
        || (line.contains("C") && line.contains("X"))
    {
        6
    } else {
        0
    }
}
