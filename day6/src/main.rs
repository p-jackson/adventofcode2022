use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to open {}", file_path));
    println!("{:?}", start_of_packet(&input));
}

fn start_of_packet(input: &str) -> Option<usize> {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find_map(|(i, window)| {
            let set = HashSet::<char>::from_iter(window.iter().cloned());

            if set.len() == 14 {
                Some(i + 14)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(start_of_packet(""), None);
        assert_eq!(start_of_packet("abc"), None);
        assert_eq!(start_of_packet("abcdefghijklma"), None);
        assert_eq!(start_of_packet("abcdefghijklmn"), Some(14));
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(
            start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(29)
        );
        assert_eq!(
            start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(26)
        );
    }
}
