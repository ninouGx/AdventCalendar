use progress_timer::time_function;

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let is_test = true;

    let input = aoc_utils::get_input_for_day(is_test);

    let result_part_1 = time_function("Part 1", 5, || part1(&input));
    println!("Part 1: {}", result_part_1);
    let result_part_2 = time_function("Part 2", 5, || part2(&input));
    println!("Part 2: {}", result_part_2);
}
