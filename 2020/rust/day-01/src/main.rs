use std::time::Instant;
use std::collections::{HashMap};

type AnyError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, AnyError>;

fn main() -> Result<()> {
    println!("----------------------------------");
    println!("Advent of Code 2020 - Day 1");

    let path = "./input/input.txt";
    let numbers = read_input(path)?;
    let numbers2 = numbers.to_vec();
    let mut numbers_map:HashMap<usize, usize> = HashMap::new();
    for x in 0..numbers.len() {
        numbers_map.insert(numbers[x], numbers[x]);
    }
    let start = Instant::now();

    println!("----------------------------------");

    let solution = simple(numbers);
    println!("Simple Solution: {}", solution);

    let duration = start.elapsed();
    println!("Time: {}µs", duration.as_micros());

    println!("----------------------------------");

    let start2 = Instant::now();

    let solution2 = complements(numbers2);
    println!("Complements Solution: {}", solution2);

    let duration2 = start2.elapsed();
    println!("Time: {}µs", duration2.as_micros());

    println!("----------------------------------");

    let start3 = Instant::now();

    let solution3 = complements_v2(numbers_map);
    println!("Complements Solution: {}", solution3);

    let duration3 = start3.elapsed();
    println!("Time: {}µs", duration3.as_micros());

    println!("----------------------------------");
    Ok(())
}

fn read_input(path: &str) -> Result<Vec<usize>> {
    std::fs::read_to_string(path)?
        .lines()
        .map(|s| s.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

fn simple(numbers: Vec<usize>) -> usize {
    //loop through the numbers, checking each other number to see if it sums to 2020
    //n squared
    for x in 0..numbers.len() {
        for y in 0..numbers.len() {
            if numbers[x] + numbers[y] == 2020 {
                println!("Identified {} and {}", numbers[x], numbers[y]);
                return numbers[x] * numbers[y];
            }
        }
    }
    panic!("solution not found")
}

fn complements(numbers: Vec<usize>) -> usize {
    // loop through once, identifying the necessary second number to make 2020 for each number
    // loop through a second time, seeking a match, using contains
    // could be faster to use hash sets and intersection...sets in general would be wise to use instead /shrug
    let mut complements: Vec<usize> = Vec::new();

    for x in 0..numbers.len() {
        complements.push(2020 - numbers[x]);
    }

    for num in numbers {
        if complements.contains(&num) {
            let other_num = 2020-num;
            println!("Identified {} and {}", num, other_num);

            return num * other_num;
        }
    }

    panic!("No solution");
}

fn complements_v2(numbers: HashMap<usize, usize>) ->usize {
    // switched to a map and optimized to a single pass

    for num in numbers.keys() {
        let other_num = 2020-num;
        if numbers.contains_key(&other_num) {
            let other_num = 2020-num;
            println!("Identified {} and {}", num, other_num);

            return num * other_num;
        }
    }

    panic!("No solution");
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_example_input() {
        let solution = simple(read_input("./input/example.txt").unwrap());
        assert_eq!(solution, 514579)
    }
}