use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read {}", file_path));

    let first = file_contents.lines().step_by(3);
    let second = file_contents.lines().skip(1).step_by(3);
    let third = file_contents.lines().skip(2).step_by(3);

    let total_score = first
        .zip(second)
        .zip(third)
        .filter_map(|((a, b), c)| get_group_badge(a, b, c))
        .filter_map(get_item_priority)
        .sum::<u32>();

    println!("{}", total_score);
}

fn get_group_badge(first: &str, second: &str, third: &str) -> Option<char> {
    let first = first.chars().collect::<HashSet<_>>();
    let second = second.chars().collect::<HashSet<_>>();
    let third = third.chars().collect::<HashSet<_>>();

    let first_and_second = first.intersection(&second).cloned().collect::<HashSet<_>>();
    first_and_second.intersection(&third).cloned().next()
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
    fn test_group_badge() {
        assert_eq!(get_group_badge("a", "a", "a"), Some('a'));
        assert_eq!(get_group_badge("a", "a", "b"), None);
        assert_eq!(get_group_badge("a", "a", "A"), None);
        assert_eq!(get_group_badge("abcdef", "cat", "a"), Some('a'));
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
