// Advent of Code 2022
// Day 6

use anyhow::Result;
use std::io;

fn window_as_bitstring(string: &[u8]) -> u32 {
    string.iter().fold(0, |acc, v| acc | 1 << (v - b'a') as u32)
}

fn detect(input: &str, window_size: usize) -> u32 {
    for (i, w) in input.as_bytes().windows(window_size).enumerate() {
        if window_as_bitstring(w).count_ones() == window_size as u32 {
            return (i + window_size) as u32;
        }
    }

    0
 }

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let now = std::time::Instant::now();
    let processed = detect(&input, 4);
    let elapsed_processed = now.elapsed();
    println!("{processed}, {:?}", elapsed_processed);

    let now = std::time::Instant::now();
    let som_processed = detect(&input, 14);
    let elapsed_som = now.elapsed();
    println!("{som_processed}, {:?}", elapsed_som);

    println!("total time: {:?}", elapsed_processed + elapsed_som);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitstring1() {
        let string = b"aaaa";
        assert_eq!(window_as_bitstring(string).count_ones(), 1)
    }

    #[test]
    fn bitstring2() {
        let string = b"abaa";
        assert_eq!(window_as_bitstring(string).count_ones(), 2)
    }

    #[test]
    fn bitstring3() {
        let string = b"abca";
        assert_eq!(window_as_bitstring(string).count_ones(), 3)
    }

    #[test]
    fn bitstring4() {
        let string = b"abcd";
        assert_eq!(window_as_bitstring(string).count_ones(), 4)
    }

    #[test]
    fn sample1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(detect(input, 4), 7);
    }

    #[test]
    fn sample2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(detect(input, 4), 5);
    }

    #[test]
    fn sample3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(detect(input, 4), 6);
    }

    #[test]
    fn sample4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(detect(input, 4), 10);
    }

    #[test]
    fn sample5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(detect(input, 4), 11);
    }
}
