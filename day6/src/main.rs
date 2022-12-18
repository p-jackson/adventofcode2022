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
        .windows(4)
        .enumerate()
        .find_map(|(i, window)| {
            let set = HashSet::<char>::from_iter(window.iter().cloned());

            if set.len() == 4 {
                Some(i + 4)
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
        assert_eq!(start_of_packet("abccd"), None);
        assert_eq!(start_of_packet("abcd"), Some(4));
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(
            start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }
}
