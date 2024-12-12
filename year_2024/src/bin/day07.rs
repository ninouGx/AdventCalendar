use std::str::FromStr;

use progress_timer::time_function;

#[derive(Clone)]
struct DataLine {
    result: usize,
    values: Vec<usize>,
}

impl FromStr for DataLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        let result = parts[0].parse().map_err(|_| "Failed to parse result as number".to_string())?;

        let values = parts[1]
            .split_ascii_whitespace()
            .map(|nb| nb.parse().unwrap())
            .collect();

        Ok(DataLine { result, values })
    }
}

fn get_result_if_valid_operator_exist(line: DataLine, is_third_operator: bool) -> usize {
    let mut results = Vec::new();
    let size = line.values.len();
    let total_combinations;

    if !is_third_operator {
        total_combinations = 1 << (size - 1);
    } else {
        total_combinations = (3_usize).pow((size as u32) - 1);
    }

    for combination in 0..total_combinations {
        if !is_third_operator {
            results.push(get_combination(size, &line, combination));
        } else {
            results.push(get_combination_with_concatenation(size, &line, combination));
        }
    }

    results
        .iter()
        .any(|&result| line.result == result)
        .then(|| line.result)
        .unwrap_or(0)
}

fn get_combination(size: usize, line: &DataLine, combination: usize) -> usize {
    let mut result: usize = line.values[0];

    for i in 0..size - 1 {
        let next_nb = line.values[i + 1];
        if (combination & (1 << i)) == 0 {
            result += next_nb;
        } else {
            result *= next_nb;
        }
    }

    result
}

fn get_combination_with_concatenation(
    size: usize,
    line: &DataLine,
    mut combination: usize
) -> usize {
    let mut result: usize = line.values[0];

    for i in 0..size - 1 {
        let next_nb = line.values[i + 1];
        let operation = combination % 3;

        if result > line.result {
            break;
        }

        match operation {
            0 => {
                result += next_nb;
            }

            1 => {
                result *= next_nb;
            }

            2 => {
                let str_result = result.to_string() + &next_nb.to_string();
                result = str_result.parse().unwrap();
            }
            _ => unreachable!(),
        }

        combination /= 3;
    }

    result
}

fn part1(input: &str) -> usize {
    let operations: Vec<DataLine> = input
        .lines()
        .map(|line| DataLine::from_str(line).unwrap())
        .collect();

    operations
        .iter()
        .map(|operation| get_result_if_valid_operator_exist(operation.clone(), false))
        .sum()
}

fn part2(input: &str) -> usize {
    let operations: Vec<DataLine> = input
        .lines()
        .map(|line| DataLine::from_str(line).unwrap())
        .collect();

    operations
        .iter()
        .map(|operation| get_result_if_valid_operator_exist(operation.clone(), true))
        .sum()
}

fn main() {
    let is_test = false;

    let input = aoc_utils::get_input_for_day(is_test);

    let result_part_1 = time_function("Part 1", 5, || part1(&input));
    println!("Part 1: {}", result_part_1);
    let result_part_2 = time_function("Part 2", 5, || part2(&input));
    println!("Part 2: {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
