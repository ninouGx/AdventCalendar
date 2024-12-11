use std::collections::HashSet;
use std::ops::Add;
use std::time::Instant;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn movement(&self) -> Coordinate {
        match self {
            Direction::UP => Coordinate { row: -1, col: 0 },
            Direction::RIGHT => Coordinate { row: 0, col: 1 },
            Direction::DOWN => Coordinate { row: 1, col: 0 },
            Direction::LEFT => Coordinate { row: 0, col: -1 },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum MapElement {
    EMPTY,
    OBSTACLE,
    PATH,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    row: isize,
    col: isize,
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coordinate { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

struct MapData {
    matrix: Vec<Vec<MapElement>>,
    pos: Coordinate,
}

impl MapData {
    fn new(input: &str) -> Self {
        let mut coord = Coordinate { row: 0, col: 0 };
        let map = input
            .lines()
            .enumerate()
            .map(|(x, line)|
                line
                    .chars()
                    .enumerate()
                    .map(|(y, c)| {
                        match c {
                            '^' => {
                                coord = Coordinate { row: x as isize, col: y as isize };
                                MapElement::PATH
                            }
                            '#' => MapElement::OBSTACLE,
                            _ => MapElement::EMPTY,
                        }
                    })
                    .collect()
            )
            .collect();

        MapData { matrix: map, pos: coord }
    }
}

fn move_in_direction(dir: Direction, map_data: &mut MapData) -> bool {
    let vector = dir.movement();
    let upcoming_pos = map_data.pos + vector;
    let can_move =
        map_data.matrix[upcoming_pos.row as usize][upcoming_pos.col as usize] !=
        MapElement::OBSTACLE;
    if can_move {
        map_data.matrix[map_data.pos.row as usize][map_data.pos.col as usize] = MapElement::PATH;
        map_data.pos = upcoming_pos;
    }
    can_move
}

fn can_move_outside(dir: Direction, map_data: &mut MapData) -> bool {
    let rows = map_data.matrix.len() as isize;
    let cols = map_data.matrix[0].len() as isize;
    let pos = map_data.pos + dir.movement();

    let can_go_outside = pos.row < 0 || pos.row >= rows || pos.col < 0 || pos.col >= cols;

    if can_go_outside {
        map_data.matrix[map_data.pos.row as usize][map_data.pos.col as usize] = MapElement::PATH;
    }

    can_go_outside
}

fn part1(input: &str) -> usize {
    let mut current_dir = Direction::UP;
    let mut map = MapData::new(input);
    while !can_move_outside(current_dir, &mut map) {
        if !move_in_direction(current_dir, &mut map) {
            current_dir = current_dir.rotate_right();
        }
    }

    map.matrix
        .iter()
        .flat_map(|line| line.iter().filter(|&elem| *elem == MapElement::PATH))
        .count()
}

fn is_loop_with_obstacle(input: &str, obstacle_pos: Coordinate) -> bool {
    let mut map = MapData::new(input);

    if obstacle_pos == map.pos {
        return false;
    }

    map.matrix[obstacle_pos.row as usize][obstacle_pos.col as usize] = MapElement::OBSTACLE;

    let mut visited = HashSet::new();
    let mut current_dir = Direction::UP;

    loop {
        let current_state = (map.pos, current_dir);
        if visited.contains(&current_state) {
            return true;
        }
        visited.insert(current_state);

        if can_move_outside(current_dir, &mut map) {
            return false;
        }

        if !move_in_direction(current_dir, &mut map) {
            current_dir = current_dir.rotate_right();
        }
    }
}

fn part2(input: &str) -> usize {
    let map_data = MapData::new(input);
    let empty_positions: Vec<Coordinate> = map_data.matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &elem)| elem == MapElement::EMPTY)
                .map(move |(col, _)| Coordinate {
                    row: row as isize,
                    col: col as isize,
                })
        })
        .collect();

    empty_positions
        .into_par_iter()
        .filter(|&pos| is_loop_with_obstacle(input, pos))
        .count()
}

fn main() {
    let is_test = false;

    let input = aoc_utils::get_input_for_day(is_test);

    let start_part1 = Instant::now();
    println!("Part 1: {}", part1(&input));
    let duration_part1 = start_part1.elapsed();
    println!("Time taken for Part 1: {:?}", duration_part1);

    let start_part2 = Instant::now();
    println!("Part 2: {}", part2(&input));
    let duration_part2 = start_part2.elapsed();
    println!("Time taken for Part 2: {:?}", duration_part2);
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
