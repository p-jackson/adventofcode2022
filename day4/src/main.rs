use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "input.txt";
    let f = File::open(file_path)
        .expect(format!("Should have been able to read {}", file_path).as_str());
    let reader = BufReader::new(f);

    let count = reader
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| parse(&line))
        .filter(fully_overlaps)
        .count();

    println!("{}", count);
}

fn parse(line: &str) -> Option<((u32, u32), (u32, u32))> {
    let mut ranges = line.split(',');
    Some((parse_part(ranges.next()?)?, parse_part(ranges.next()?)?))
}

fn parse_part(range: &str) -> Option<(u32, u32)> {
    let mut parts = range.split('-');
    let small = parts.next()?.parse::<u32>().ok()?;
    let big = parts.next()?.parse::<u32>().ok()?;
    Some((small, big))
}

fn fully_overlaps(partners: &((u32, u32), (u32, u32))) -> bool {
    compare_part(partners.0, partners.1) || compare_part(partners.1, partners.0)
}

fn compare_part(first: (u32, u32), second: (u32, u32)) -> bool {
    first.0 <= second.0 && first.1 >= second.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_badge() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("a-b,c-d"), None);
        assert_eq!(parse("1-2,3-4"), Some(((1, 2), (3, 4))));
    }

    #[test]
    fn test_fully_overlaps() {
        assert!(fully_overlaps(&((1, 2), (1, 2))));
        assert!(!fully_overlaps(&((2, 4), (6, 8))));
        assert!(!fully_overlaps(&((2, 3), (4, 5))));
        assert!(!fully_overlaps(&((5, 7), (7, 9))));
        assert!(fully_overlaps(&((2, 8), (3, 7))));
        assert!(fully_overlaps(&((6, 6), (4, 6))));
        assert!(!fully_overlaps(&((2, 6), (4, 8))));
    }
}
