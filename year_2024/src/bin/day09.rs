use std::time::Duration;

use progress_timer::time_function;

#[derive(Clone)]
struct Hole {
    index: usize,
    size: usize,
}

impl Hole {
    fn new(index: usize, size: usize) -> Self {
        Self { index, size }
    }
}

fn create_nb_list(char_id: char, input_nb: Option<usize>) -> Vec<Option<usize>> {
    let nb_of_iter = char_id.to_digit(10).unwrap() as usize;
    vec![input_nb; nb_of_iter]
}

fn compute_disk(disk: Vec<char>) -> Vec<Option<usize>> {
    let mut is_free_space: bool = false;
    let mut id: usize = 0;
    disk.iter()
        .flat_map(|&char| {
            let result = create_nb_list(char, if is_free_space { None } else { Some(id) });
            is_free_space = !is_free_space;
            if is_free_space {
                id += 1;
            }
            result
        })
        .collect()
}

fn does_compute_contains_hole(compute: &[Option<usize>]) -> bool {
    let mut found_hole = false;
    compute.iter().any(|&nb| {
        if nb.is_none() {
            found_hole = true;
            false
        } else {
            found_hole
        }
    })
}

fn get_holes_files(compute: &[Option<usize>]) -> (Vec<Hole>, Vec<Hole>) {
    let mut holes: Vec<Hole> = Vec::new();
    let mut files: Vec<Hole> = Vec::new();
    let mut is_in_hole: bool = false;
    let mut hole_size = 0;
    let mut file_size = 0;
    let size = compute.len();
    let mut last_nb: Option<usize> = None;
    // dbg!(compute.iter().enumerate().collect::<Vec<_>>());
    for i in 0..size {
        let nb = compute[i];

        if nb.is_none() {
            if !is_in_hole {
                files.push(Hole::new(i - file_size, file_size));
                file_size = 0;
            }
            is_in_hole = true;
            hole_size += 1;
        } else {
            if is_in_hole {
                holes.push(Hole::new(i - hole_size, hole_size));
                hole_size = 0;
            }
            is_in_hole = false;
            file_size += 1;

            if last_nb.is_some() && last_nb.unwrap() != nb.unwrap() {
                file_size -= 1;
                files.push(Hole::new(i - file_size, file_size));
                file_size = 1;
            }
        }
        last_nb = nb;
    }

    if file_size > 0 {
        files.push(Hole::new(size - file_size, file_size));
    }
    if hole_size > 0 {
        holes.push(Hole::new(size - hole_size, hole_size));
    }
    (holes, files)
}

fn part1(input: &str) -> usize {
    let disk: Vec<char> = input.chars().collect();
    let mut computed: Vec<Option<usize>> = compute_disk(disk);

    let moves: Vec<_> = {
        let nbs: Vec<_> = computed
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, nb)| nb.is_some())
            .rev()
            .collect();

        let voids: Vec<_> = computed
            .iter()
            .enumerate()
            .filter(|(_, nb)| nb.is_none())
            .collect();

        voids
            .iter()
            .zip(nbs.iter())
            .map(|(v, n)| (v.0, n.0, n.1))
            .collect::<Vec<_>>()
    };

    for (void_idx, nb_idx, value) in moves {
        if does_compute_contains_hole(&computed) {
            computed[void_idx] = value;
            computed[nb_idx] = None;
        } else {
            break;
        }
    }

    computed
        .iter()
        .enumerate()
        .filter_map(|(idx, &nb)| nb.map(|n| idx * n))
        .sum()
}

fn part2(input: &str) -> usize {
    let disk: Vec<char> = input.chars().collect();
    let computed: Vec<Option<usize>> = compute_disk(disk);
    let (mut holes, mut files) = get_holes_files(&computed);

    for i in (0..files.len()).rev() {
        for j in 0..holes.len() {
            let file = &mut files[i];
            let hole = &mut holes[j];

            if file.index > hole.index && file.size <= hole.size {
                file.index = hole.index;
                hole.index += file.size;
                hole.size -= file.size;
                if hole.size == 0 {
                    holes.remove(j);
                    break;
                }
                holes.sort_by(|h1, h2| h1.index.cmp(&h2.index));
            }
        }
    }
    files
        .iter()
        .enumerate()
        .map(|(idx, file)| {
            let mut sum = 0;
            for i in file.index..file.index + file.size {
                let product = i * idx;
                sum += product;
            }
            sum
        })
        .sum()
}

fn main() {
    let is_test = false;

    let input = aoc_utils::get_input_for_day(is_test);

    let result_part_1 = time_function(
        "Part 1",
        Duration::from_secs(5),
        Duration::from_millis(100),
        || part1(&input)
    );
    println!("Part 1: {}", result_part_1);
    let result_part_2 = time_function(
        "Part 2",
        Duration::from_secs(5),
        Duration::from_millis(100),
        || part2(&input)
    );
    println!("Part 2: {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform_string_to_compute(str: &str) -> Vec<Option<usize>> {
        str.chars()
            .map(|c| {
                if c == '.' { None } else { c.to_digit(10).map(|d| d as usize) }
            })
            .collect()
    }

    #[test]
    fn test_part1_example1() {
        let input = "112";
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_base_example() {
        let input = "1";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_compute() {
        let input = "12345";
        let expected_compute = "0..111....22222";

        let disk: Vec<char> = input.chars().collect();

        let input_compute = compute_disk(disk);

        assert_eq!(input_compute, transform_string_to_compute(expected_compute));
    }

    #[test]
    fn test_compute2() {
        let input = "1234";
        let expected_compute = "0..111....";

        let disk: Vec<char> = input.chars().collect();

        let input_compute = compute_disk(disk);

        assert_eq!(input_compute, transform_string_to_compute(expected_compute));
    }
}
