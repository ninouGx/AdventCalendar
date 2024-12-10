use std::ops::Add;

#[derive(Copy, Clone, Debug)]
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

#[derive(PartialEq)]
enum MapElement {
    EMPTY,
    OBSTACLE,
    PATH,
}

#[derive(Copy, Clone)]
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

fn retrieve_map(input: &str) -> MapData {
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
    let mut map: MapData = retrieve_map(input);
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

fn part2(_input: &str) -> usize {
    0
}

fn main() {
    let is_test = false;

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
