use std::{ collections::{ HashMap, HashSet, VecDeque }, str::FromStr, time::Duration };
use progress_timer::time_function;
use rayon::iter::{ IntoParallelRefIterator, ParallelIterator };
use std::sync::Mutex;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
struct Pos(u32, u32);
type NineSet = HashSet<Pos>;

struct Grid {
    map: HashMap<Pos, u8>,
    zeros: HashSet<Pos>,
    nines: HashSet<Pos>,
}

impl Grid {
    fn get_neighbors(&self, pos: Pos) -> Vec<Pos> {
        let Pos(x, y) = pos;
        let mut neighbors = Vec::new();
        neighbors.push(Pos(x.saturating_add(1), y));
        if x > 0 {
            neighbors.push(Pos(x - 1, y));
        }
        neighbors.push(Pos(x, y.saturating_add(1)));
        if y > 0 {
            neighbors.push(Pos(x, y - 1));
        }
        neighbors
    }

    fn get_neighbors_minus_one(&self, pos: Pos) -> Vec<Pos> {
        if self.zeros.contains(&pos) {
            return vec![];
        }

        let height = if self.nines.contains(&pos) { 9 } else { *self.map.get(&pos).unwrap() };

        self.get_neighbors(pos)
            .iter()
            .filter(|&p| {
                let neighbor_height = if self.zeros.contains(p) {
                    Some(0)
                } else {
                    self.map.get(p).map(|&h| h)
                };
                neighbor_height.is_some() && neighbor_height.unwrap() == height - 1
            })
            .cloned()
            .collect()
    }

    fn count_path(&self, start_pos: Pos) -> usize {
        let mut visited: HashSet<Pos> = HashSet::new();
        self.dfs(start_pos, &mut visited)
    }

    fn dfs(&self, current_pos: Pos, visited: &mut HashSet<Pos>) -> usize {
        if self.nines.contains(&current_pos) {
            return 1;
        }

        let current_height = if self.zeros.contains(&current_pos) {
            0
        } else {
            *self.map.get(&current_pos).unwrap()
        };

        visited.insert(current_pos);

        let mut nb_paths: usize = 0;
        for next_pos in self.get_neighbors(current_pos) {
            if !visited.contains(&next_pos) {
                let next_height = if self.nines.contains(&next_pos) {
                    Some(9)
                } else {
                    self.map.get(&next_pos).copied()
                };

                if next_height == Some(current_height + 1) {
                    nb_paths += self.dfs(next_pos, visited);
                }
            }
        }

        visited.remove(&current_pos);

        nb_paths
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut zeros: HashSet<Pos> = HashSet::new();
        let mut nines: HashSet<Pos> = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit()) {
                let pos = Pos(y as u32, x as u32);
                let value = c.to_digit(10).ok_or_else(|| format!("Invalid digit: {}", c))?;
                match value {
                    0 => {
                        zeros.insert(pos);
                    }
                    9 => {
                        nines.insert(pos);
                    }
                    _ => {
                        map.insert(pos, value as u8);
                    }
                }
            }
        }

        Ok(Grid { map, zeros, nines })
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let rechable_nines: Mutex<HashMap<Pos, NineSet>> = Mutex::new(HashMap::new());

    grid.nines.par_iter().for_each(|&nine| {
        let mut local_reachable: HashMap<Pos, NineSet> = HashMap::new();
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut queue: VecDeque<Pos> = VecDeque::new();
        queue.push_back(nine);

        local_reachable.entry(nine).or_insert_with(HashSet::new).insert(nine);

        while let Some(current_pos) = queue.pop_front() {
            let neighbors = grid.get_neighbors_minus_one(current_pos);
            for neighbor in neighbors {
                if visited.insert(neighbor) {
                    local_reachable.entry(neighbor).or_insert_with(HashSet::new).insert(nine);

                    queue.push_back(neighbor);
                }
            }
        }

        let mut global = rechable_nines.lock().unwrap();
        for (pos, nines) in local_reachable {
            global.entry(pos).or_insert_with(HashSet::new).extend(nines);
        }
    });

    // let mut sorted_nines: Vec<_> = rechable_nines.lock().unwrap().clone().into_iter().collect();
    // sorted_nines.sort_by_key(|(pos, _)| *pos);
    // dbg!(&sorted_nines);

    // dbg!(
    //     sorted_nines
    //         .iter()
    //         .filter(|(pos, _)| grid.zeros.contains(pos))
    //         .collect::<Vec<_>>()
    // );

    let final_map = rechable_nines.lock().unwrap();
    grid.zeros
        .iter()
        .map(|zero| final_map.get(zero).map_or(0, |nines| nines.len()))
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.zeros
        .par_iter()
        .map(|&zero| grid.count_path(zero))
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

    #[test]
    fn test_part1_example1() {
        let input = "\
0123456789";
        let result = part1(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part1_example2() {
        let input = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_example3() {
        let input = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_no_score() {
        let input = "\
950";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example1() {
        let input = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
        let result = part2(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_example2() {
        let input = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let result = part2(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2_example3() {
        let input = "\
012345
123456
234567
345678
4.6789
56789.";
        let result = part2(input);
        assert_eq!(result, 227);
    }

    #[test]
    fn test_part2_no_score() {
        let input = "\
012345679";
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
