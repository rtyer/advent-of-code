#[macro_use]
extern crate lazy_static;

use std::fmt;
use std::time::Instant;

use regex::Regex;

lazy_static! {
    static ref PASSWORD_RE:regex::Regex = Regex::new(r"(\d+)-(\d+)\s+([a-zA-Z])+:\s(\S+)").unwrap();
}

type AnyError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, AnyError>;

fn main() -> Result<()> {
    println!("----------------------------------");
    println!("Advent of Code 2020 - Day 2");
    println!("----------------------------------");
    // example data;
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    // read file in, for each line, parse as follows.
    // (lower)-(upper) (letter): (candidate)
    // (\d+)-(\d+)\s+([a-zA-Z])+:\s(\S+)
    let path = "./input/input.txt";

    let start = Instant::now();

    let count_part_1 = std::fs::read_to_string(path)?
        .lines()
        .map(|line| parse_line(line))
        .map(|(candidate, rule)| is_valid(&candidate, rule))
        .filter(|validity| *validity == true)
        .count();

    let duration = start.elapsed();

    let start2 = Instant::now();

    let count_part_2 = std::fs::read_to_string(path)?
        .lines()
        .map(|line| parse_line(line))
        .map(|(candidate, rule)| is_valid_part_two(&candidate, rule))
        .filter(|validity| *validity == true)
        .count();

    let duration2 = start2.elapsed();
    println!("----------------------------------");
    println!("Part 1");
    println!("Identified {} valid passwords", count_part_1);
    println!("Took {}ms", duration.as_millis());
    println!("----------------------------------");
    println!("Part 2");
    println!("Identified {} valid passwords", count_part_2);
    println!("Took {}ms", duration2.as_millis());
    println!("----------------------------------");

    Ok(())
}

#[derive(Debug, PartialEq)]
struct PasswordRule {
    first: usize,
    second: usize,
    letter: char,
}

impl fmt::Display for PasswordRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "PasswordRule(lower:{}, upper:{}, leter:{})", self.first, self.second, self.letter)
    }
}

fn parse_line(line: &str) -> (String, PasswordRule) {
    let caps = PASSWORD_RE.captures(line).unwrap();
    let rule = PasswordRule { first: caps[1].parse().unwrap(), second: caps[2].parse().unwrap(), letter: char_from_string(&caps[3]) };
    return (String::from(&caps[4]), rule);
}

fn is_valid(password: &str, rule: PasswordRule) -> bool {
    let occurrences = password.chars().fold(0, |count, c|
        if c == rule.letter { count + 1 } else { count });
    let valid = occurrences >= rule.first && occurrences <= rule.second;

    println!("Password {} with rules {} valid? {}", password, rule, valid);

    return valid;
}

fn is_valid_part_two(password: &str, rule: PasswordRule) -> bool {
    let characters: Vec<char> = password.chars().collect();
    let valid =
        (characters[rule.first - 1] == rule.letter && characters[rule.second - 1] != rule.letter)
            || (characters[rule.first - 1] != rule.letter && characters[rule.second - 1] == rule.letter);

    println!("Password {} with rules {} valid? {}", password, rule, valid);

    return valid;
}

fn char_from_string(str: &str) -> char {
    let chars: Vec<char> = str.chars().collect();
    if chars.len() != 1 {
        panic!("expected single password char");
    }
    return chars[0];
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_is_valid_part1() {
        assert_eq!(true, is_valid("abcde", PasswordRule { first: 1, second: 3, letter: 'a' }));
        assert_eq!(false, is_valid("cdefg", PasswordRule { first: 1, second: 3, letter: 'b' }));
        assert_eq!(true, is_valid("ccccccccc", PasswordRule { first: 2, second: 9, letter: 'c' }));
        assert_eq!(false, is_valid("c", PasswordRule { first: 2, second: 9, letter: 'c' }));
    }


    #[test]
    fn test_is_valid_part2() {
        assert_eq!(true, is_valid_part_two("abcde", PasswordRule { first: 1, second: 3, letter: 'a' }));
        assert_eq!(false, is_valid_part_two("cdefg", PasswordRule { first: 1, second: 3, letter: 'b' }));
        assert_eq!(false, is_valid_part_two("ccccccccc", PasswordRule { first: 2, second: 9, letter: 'c' }));
    }

    #[test]
    fn test_parse_line() {
        // 1-3 a: abcde
        // 1-3 b: cdefg
        // 2-9 c: ccccccccc
        let (pw1, rule1) = parse_line("1-3 a: abcde");
        assert_eq!(pw1, "abcde");
        assert_eq!(rule1, PasswordRule { first: 1, second: 3, letter: 'a' });
        let (pw2, rule2) = parse_line("1-3 b: cdefg");
        assert_eq!(pw2, "cdefg");
        assert_eq!(rule2, PasswordRule { first: 1, second: 3, letter: 'b' });
        let (pw3, rule3) = parse_line("2-9 c: ccccccccc");
        assert_eq!(pw3, "ccccccccc");
        assert_eq!(rule3, PasswordRule { first: 2, second: 9, letter: 'c' });
    }
}