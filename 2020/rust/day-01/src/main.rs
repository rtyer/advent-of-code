use std::collections::HashMap;
use std::time::Instant;

type AnyError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, AnyError>;

fn main() -> Result<()> {
    println!("----------------------------------");
    println!("Advent of Code 2020 - Day 1");

    let path = "./input/input.txt";
    let numbers = read_input(path)?;
    let numbers_map = read_input_map(path)?;
    let numbers_map_two = numbers_map.clone();

    let start = Instant::now();
    println!("----------------------------------");
    println!("Part One");
    println!("----------------------------------");

    let solution = simple(numbers);
    println!("Simple Solution: {}", solution);

    let duration = start.elapsed();
    println!("Time: {}µs", duration.as_micros());

    println!("----------------------------------");

    let start_complements = Instant::now();

    let complements_solution = complements(numbers_map);
    println!("Complements Solution: {}", complements_solution);

    let duration_complements = start_complements.elapsed();
    println!("Time: {}µs", duration_complements.as_micros());

    println!("----------------------------------");
    println!("Part Two");
    println!("----------------------------------");
    let start_part_two = Instant::now();

    let part_two_solution = part_two(numbers_map_two);
    println!("Complements Solution: {}", part_two_solution);

    let duration_part_two = start_part_two.elapsed();
    println!("Time: {}µs", duration_part_two.as_micros());

    Ok(())
}

fn read_input(path: &str) -> Result<Vec<usize>> {
    std::fs::read_to_string(path)?
        .lines()
        .map(|s| s.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

fn read_input_map(path: &str) -> Result<HashMap<usize, usize>> {
    let numbers = read_input(path)?;
    let mut numbers_map: HashMap<usize, usize> = HashMap::new();
    for x in 0..numbers.len() {
        numbers_map.insert(numbers[x], numbers[x]);
    }
    return Ok(numbers_map);
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

fn complements(numbers: HashMap<usize, usize>) -> usize {
    for num in numbers.keys() {
        let other_num = 2020 - num;
        if numbers.contains_key(&other_num) {
            println!("Identified {} and {}", num, other_num);

            return num * other_num;
        }
    }

    panic!("No solution");
}

fn part_two(numbers: HashMap<usize, usize>) -> usize {
    for num in numbers.keys() {
        let remainder_sum = 2020 - num;
        for num2 in numbers.keys() {
            if num2 != num && &remainder_sum > num2 { //verifying remainder greater than sum due to unsigned ints
                let other_num = remainder_sum - num2;
                if numbers.contains_key(&other_num) {
                    println!("Identified {} and {} and {}", num, num2, other_num);

                    return num * other_num * num2;
                }
            }
        }
    }

    panic!("No solution");
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_example_input_simple() {
        let solution = simple(read_input("./input/example.txt").unwrap());
        assert_eq!(solution, 514579)
    }

    #[test]
    fn test_example_input_complements_v2() {
        let solution = complements(read_input_map("./input/example.txt").unwrap());
        assert_eq!(solution, 514579)
    }

    #[test]
    fn test_example_input_part_two() {
        let map = read_input_map("./input/example.txt").unwrap();

        let solution = part_two(map);
        assert_eq!(solution, 241861950);
    }
}