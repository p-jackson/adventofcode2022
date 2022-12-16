use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path)
        .expect(format!("Should have been able to read {}", file_path).as_str());
    let reader = BufReader::new(file);

    let total_score = reader
        .lines()
        .filter_map(|result| result.ok())
        .filter_map(get_misplaced_item)
        .filter_map(get_item_priority)
        .sum::<u32>();

    println!("{}", total_score);
}

fn get_misplaced_item(line: String) -> Option<char> {
    // This assumes characters are ascii
    let half = line.len() / 2;

    let items_in_first_half = line.chars().take(half).collect::<HashSet<_>>();
    line.chars()
        .skip(half)
        .find(|c| items_in_first_half.contains(c))
}

fn get_item_priority(item: char) -> Option<u32> {
    match item {
        'a'..='z' => Some(item as u32 - 96),
        'A'..='Z' => Some(item as u32 - 38),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_item() {
        assert_eq!(get_misplaced_item("aa".to_string()), Some('a'));
        assert_eq!(get_misplaced_item("abca".to_string()), Some('a'));
        assert_eq!(get_misplaced_item("abac".to_string()), Some('a'));
        assert_eq!(get_misplaced_item("abBa".to_string()), Some('a'));
        assert_eq!(get_misplaced_item("".to_string()), None);
        assert_eq!(get_misplaced_item("abcd".to_string()), None);
    }

    #[test]
    fn test_add() {
        assert_eq!(get_item_priority('a'), Some(1));
        assert_eq!(get_item_priority('z'), Some(26));
        assert_eq!(get_item_priority('A'), Some(27));
        assert_eq!(get_item_priority('Z'), Some(52));
        assert_eq!(get_item_priority('-'), None);
    }
}
