use std::{fs, num::ParseIntError};

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("Should have been able to read {}", file_path).as_str());

    let mut current_maxes = vec![];
    let elves = file_contents.split("\n\n");
    for elf in elves {
        let meals = elf
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, ParseIntError>>()
            .expect("Contained invalid integer");
        let total_calories: u32 = meals.iter().sum();

        if current_maxes.len() < 4 {
            current_maxes.push(total_calories);
        } else {
            current_maxes[0] = total_calories;
            current_maxes.sort_unstable();
        }
    }

    println!("{}\n", current_maxes.iter().skip(1).sum::<u32>());
}
