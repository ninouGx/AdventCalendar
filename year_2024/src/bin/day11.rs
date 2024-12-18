use std::{ collections::HashMap, str::FromStr, time::Duration };
use progress_timer::time_function;
use rayon::iter::{ IntoParallelRefIterator, ParallelIterator };

struct DataLine {
    stones: Vec<u64>,
}

impl DataLine {
    fn replace(&mut self, index: u64, elem: u64) {
        self.stones.remove(index as usize);
        self.stones.insert(index as usize, elem);
    }
    fn nb_map_very_complex(&mut self, nb: u64, index: u64) -> bool {
        match nb {
            0 => {
                self.replace(index, 1);
                false
            }
            val if val.to_string().len() % 2 == 0 => {
                let val_str = val.to_string();
                let (left, right) = val_str.split_at(val_str.len() / 2);
                let left_stone = left.parse::<u64>().expect("It should parse a number (u64)");
                let right_stone = right.parse::<u64>().expect("It should parse a number (u64)");
                self.replace(index, left_stone);
                self.stones.insert((index as usize) + 1, right_stone);
                true
            }
            _ => {
                self.replace(index, nb * 2024);
                false
            }
        }
    }

    fn nb_map(&self, nb: u64) -> Vec<u64> {
        match nb {
            0 => { vec![1] }
            val if val.to_string().len() % 2 == 0 => {
                let val_str = val.to_string();
                let (left, right) = val_str.split_at(val_str.len() / 2);

                let left_stone = left.parse::<u64>().expect("It should parse a number (u64)");
                let right_stone = right.parse::<u64>().expect("It should parse a number (u64)");

                vec![left_stone, right_stone]
            }
            _ => { vec![nb * 2024] }
        }
    }

    fn do_an_iter(&mut self, nb_of_iter: u32) {
        for _iter in 0..nb_of_iter {
            let mut i = 0;
            while i < self.stones.len() {
                let nb = self.stones[i];
                if self.nb_map_very_complex(nb, i as u64) {
                    i += 1;
                }
                i += 1;
            }
            // dbg!(&self.stones);
            // println!("Doing iteration {}/{}", iter + 1, nb_of_iter);
        }
    }

    fn get_nb_of_elem(
        &self,
        nb: u64,
        iter_nb: u32,
        nb_of_iter: u32,
        cache: &mut HashMap<(u64, u32), u64>
    ) -> u64 {
        if let Some(&result) = cache.get(&(nb, iter_nb)) {
            return result;
        }

        if iter_nb == nb_of_iter {
            return 1;
        }

        let sum = self
            .nb_map(nb)
            .into_iter()
            .map(|n| self.get_nb_of_elem(n, iter_nb + 1, nb_of_iter, cache))
            .sum();

        cache.insert((nb, iter_nb), sum);
        sum
    }
}

impl FromStr for DataLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_ascii_whitespace()
            .map(|nb| nb.parse().unwrap())
            .collect();

        Ok(DataLine { stones })
    }
}

fn part1(input: &str) -> usize {
    let mut data = DataLine::from_str(input).unwrap();
    data.do_an_iter(25);
    data.stones.len()
}

fn part2(input: &str) -> usize {
    let data = DataLine::from_str(input).unwrap();
    let nb_iter = 75;
    let sum: u64 = data.stones
        .par_iter()
        .map(|&stone| {
            let mut cache = HashMap::new();
            data.get_nb_of_elem(stone, 0, nb_iter, &mut cache)
        })
        .sum();
    sum as usize
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

    #[test]
    fn test_part1_example1() {
        let input = "0 1 10 99 999";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let input = "125 17";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example1() {
        let input = "\
part2
test
input";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example2() {
        let input = "\
another
part2
case";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_edge_case() {
        let input = "\
edge
case
test";
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
