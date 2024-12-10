fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let is_test = true;

    let input = aoc_utils::get_input_for_day(is_test);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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
