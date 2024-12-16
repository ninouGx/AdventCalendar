use std::{ collections::HashMap, str::FromStr, time::Duration };
use progress_timer::time_function;

#[derive(Clone, Copy, Debug)]
struct Movement {
    drow: i32,
    dcol: i32,
}

impl Movement {
    fn mult(self, mult: i32) -> Movement {
        Movement {
            drow: self.drow * mult,
            dcol: self.dcol * mult,
        }
    }
}

impl Movement {
    fn get_reverse(&self) -> Self {
        Movement {
            drow: -self.drow,
            dcol: -self.dcol,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn get_movement_to(&self, target: Position) -> Movement {
        Movement {
            drow: (target.row as i32) - (self.row as i32),
            dcol: (target.col as i32) - (self.col as i32),
        }
    }
}

#[derive(Debug)]
struct Antenna {
    position: Position,
    frequency: char,
}

#[derive(Debug, Clone, Copy)]
struct AntennaPair {
    antenna1_idx: usize,
    antenna2_idx: usize,
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    antinodes: Vec<Position>,
    antennas: Vec<Antenna>,
}

impl Grid {
    fn make_antenna_pairs(&self) -> Vec<AntennaPair> {
        self.antennas
            .iter()
            .enumerate()
            .flat_map(|(idx_1, antenna_1)| {
                self.antennas
                    .iter()
                    .enumerate()
                    .filter(
                        move |&(idx_2, antenna_2)|
                            idx_1 < idx_2 && antenna_1.frequency == antenna_2.frequency
                    )
                    .map(move |(idx_2, _)| {
                        AntennaPair { antenna1_idx: idx_1, antenna2_idx: idx_2 }
                    })
            })
            .collect()
    }

    fn is_mov_out_of_bound(&self, pos: Position, mov: Movement) -> bool {
        let r = (pos.row as i32) + mov.drow;
        let c = (pos.col as i32) + mov.dcol;

        !(0..self.rows as i32).contains(&r) || !(0..self.cols as i32).contains(&c)
    }

    fn add_antinode(&mut self, pair: AntennaPair, is_limiting: bool) {
        let antenna1_pos = self.antennas[pair.antenna1_idx].position;
        let antenna2_pos = self.antennas[pair.antenna2_idx].position;

        let mov: Movement = antenna1_pos.get_movement_to(antenna2_pos);
        let rev_mov = mov.get_reverse();

        let mut is_valid_rev_mov = true;
        let mut is_valid_mov = true;
        let mut multi = 1;

        while (multi == 1 || !is_limiting) && (is_valid_mov || is_valid_rev_mov) {
            if is_valid_rev_mov {
                is_valid_rev_mov = self.add_antinode_position(antenna1_pos, rev_mov.mult(multi));
            }
            if is_valid_mov {
                is_valid_mov = self.add_antinode_position(antenna2_pos, mov.mult(multi));
            }
            multi += 1;
        }
    }

    fn add_antinode_position(&mut self, pos: Position, mov: Movement) -> bool {
        let is_mov_out_of_bound = self.is_mov_out_of_bound(pos, mov);
        if !is_mov_out_of_bound {
            let antinode_pos = Position {
                row: ((pos.row as i32) + mov.drow) as usize,
                col: ((pos.col as i32) + mov.dcol) as usize,
            };

            if !self.antinodes.contains(&antinode_pos) {
                self.antinodes.push(antinode_pos);
            }
        }
        !is_mov_out_of_bound
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<&str> = s.split('\n').collect();
        let cols = list[0].chars().count();
        let rows = list.len();

        let cells: Vec<Position> = Vec::new();

        let antennas: Vec<Antenna> = s
            .split('\n')
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .map(move |(col, c)| {
                        let pos = Position { row: row, col: col };
                        let antenna = Antenna { position: pos, frequency: c };
                        antenna
                    })
            })
            .collect();

        Ok(Grid { rows: rows, cols: cols, antinodes: cells, antennas: antennas })
    }
}
fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    let pairs = grid.make_antenna_pairs();
    pairs.iter().for_each(|&pair| grid.add_antinode(pair, true));

    grid.antinodes.len()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    let pairs = grid.make_antenna_pairs();
    pairs.iter().for_each(|&pair| grid.add_antinode(pair, false));

    let mut group: HashMap<char, Vec<Position>> = HashMap::new();
    grid.antennas.iter().for_each(|antenna| {
        group.entry(antenna.frequency).or_default().push(antenna.position);
    });
    let no_mov = Movement { drow: 0, dcol: 0 };
    group
        .into_values()
        .filter(|posistions| posistions.len() > 1)
        .for_each(|positions| {
            positions.iter().for_each(|&position| {
                grid.add_antinode_position(position, no_mov);
            })
        });
    //grid.antinodes.sort_by(|a1, a2| { a1.row.cmp(&a2.row).then_with(|| a1.col.cmp(&a2.col)) });

    grid.antinodes.len()
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
