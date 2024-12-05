#[derive(Copy, Clone)]
struct Direction {
    dx: i32,
    dy: i32,
}

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";
const INPUT: &str = include_str!("../data/day04_input.txt");
//const INPUT: &str = include_str!("../data/test/day04_test_input.txt");

fn is_in_bound(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    0 <= row && row < rows && 0 <= col && col < cols
}

fn is_word_at_in_direction(
    grid: &[Vec<char>],
    word: &str,
    start_row: usize,
    star_col: usize,
    dx: i32,
    dy: i32
) -> bool {
    word.chars()
        .enumerate()
        .all(|(index, ch)| {
            let x: i32 = (start_row as i32) + (index as i32) * dy;
            let y: i32 = (star_col as i32) + (index as i32) * dx;

            is_in_bound(grid, x, y) && grid[x as usize][y as usize] == ch
        })
}

fn is_word_at_in_direction_in_x(
    grid: &[Vec<char>],
    word: &str,
    start_row: usize,
    start_col: usize,
    direction_x: (Direction, Direction)
) -> bool {
    //println!("grid: {:?}", grid);
    //println!("word: {}", word);
    //println!("start_row: {}", start_row);
    //println!("star_col: {}", star_col);
    let (dir1, dir2) = direction_x;

    let shift_start_row_1: i32 = (start_row as i32) + dir1.dy;
    let shift_start_col_1: i32 = (start_col as i32) + dir1.dx;
    let shift_back_dx_1: i32 = (start_col as i32) - (shift_start_col_1 as i32);
    let shift_back_dy_1: i32 = (start_row as i32) - (shift_start_row_1 as i32);

    let shift_start_row_2: i32 = (start_row as i32) + dir2.dy;
    let shift_start_col_2: i32 = (start_col as i32) + dir2.dx;
    let shift_back_dx_2: i32 = (start_col as i32) - (shift_start_col_2 as i32);
    let shift_back_dy_2: i32 = (start_row as i32) - (shift_start_row_2 as i32);

    if
        !is_in_bound(grid, shift_start_row_1, shift_start_col_1) ||
        !is_in_bound(grid, shift_start_row_2, shift_start_col_2)
    {
        return false;
    }
    let first_half = is_word_at_in_direction(
        grid,
        word,
        shift_start_row_1 as usize,
        shift_start_col_1 as usize,
        shift_back_dx_1,
        shift_back_dy_1
    );

    let second_half = is_word_at_in_direction(
        grid,
        word,
        shift_start_row_2 as usize,
        shift_start_col_2 as usize,
        shift_back_dx_2,
        shift_back_dy_2
    );
    //println!("1: {}, 2: {}", first_half, second_half);
    first_half && second_half
}

fn nb_occurences(grid: &[Vec<char>], word: &str, directions: &[Direction]) -> usize {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .flat_map(|(row, col)| {
            directions
                .iter()
                .map(move |direction| {
                    is_word_at_in_direction(grid, word, row, col, direction.dx, direction.dy)
                })
        })
        .filter(|&found| found)
        .count()
}

fn nb_occurences_in_x(
    grid: &[Vec<char>],
    word: &str,
    directions: &[(Direction, Direction)]
) -> usize {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid[row][col] == 'A')
        .flat_map(|(row, col)| {
            directions
                .iter()
                .map(move |&direction_x| {
                    is_word_at_in_direction_in_x(grid, word, row, col, direction_x)
                })
        })
        .filter(|&found| found)
        .count()
}

fn resolve_part_one(grid: &[Vec<char>]) -> usize {
    let directions: [Direction; 8] = [
        Direction { dx: 0, dy: 1 }, // Horizontal
        Direction { dx: 1, dy: 0 }, // Vertical
        Direction { dx: 1, dy: 1 }, // Diagonal down-right
        Direction { dx: -1, dy: 1 }, // Diagonal up-right
        Direction { dx: 0, dy: -1 }, // Horizontal backwards
        Direction { dx: -1, dy: 0 }, // Vertical upwards
        Direction { dx: -1, dy: -1 }, // Diagonal up-left
        Direction { dx: 1, dy: -1 }, // Diagonal down-left
    ];

    nb_occurences(grid, XMAS, &directions)
}

fn resolve_part_two(grid: &[Vec<char>]) -> usize {
    let directions: [Direction; 4] = [
        Direction { dx: 1, dy: 1 }, // Diagonal down-right
        Direction { dx: -1, dy: 1 }, // Diagonal up-right
        Direction { dx: -1, dy: -1 }, // Diagonal up-left
        Direction { dx: 1, dy: -1 }, // Diagonal down-left
    ];

    let all_directions_pairs: Vec<(Direction, Direction)> = directions
        .iter()
        .enumerate()
        .flat_map(|(i, first_direction)| {
            directions
                .iter()
                .skip(i + 1)
                .map(move |second_direction| (*first_direction, *second_direction))
        })
        .collect();
    //println!("nb of pair: {}", all_directions_pairs.len());
    nb_occurences_in_x(grid, MAS, &all_directions_pairs)
}

fn main() {
    let grid: Vec<Vec<char>> = INPUT.lines()
        .map(|line| line.chars().collect())
        .collect();

    let nb_xmas = resolve_part_one(&grid);
    println!("Number of XMAS: {}", nb_xmas);

    let nb_x_mas = resolve_part_two(&grid);
    println!("Number of X-MAS: {}", nb_x_mas);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve_part_one() {
        let input = "\
XMAS
...A
...M
...X";
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let result = resolve_part_one(&grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_resolve_part_two() {
        let input = "\
M.S
.A.
M.S";
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let result = resolve_part_two(&grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_resolve_part_two_no_x_mas() {
        let input = "\
M.S
.A.
M..";
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let result = resolve_part_two(&grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_resolve_part_two_multiple_x_mas() {
        let input = "\
M.S.M.S
.A...A.
M.S.M.S";
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let result = resolve_part_two(&grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_resolve_part_two_three() {
        let input = "\
M.M
.A.
S.S";
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let result = resolve_part_two(&grid);
        assert_eq!(result, 1);
    }
}
