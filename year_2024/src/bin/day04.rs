const XMAS: &str = "XMAS";
const MAS: &str = "MAS";
const INPUT: &str = include_str!("../data/day04_input.txt");
//const INPUT: &str = include_str!("../data/test/day04_test_input.txt");

fn is_in_bound(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    0 <= row && row < rows && 0 <= col && col < cols
}

fn is_word_at_in_direction(
    grid: &Vec<Vec<char>>,
    word: &str,
    start_row: usize,
    star_col: usize,
    dx: i32,
    dy: i32
) -> bool {
    word.chars()
        .enumerate()
        .all(|(index, ch)| {
            let x: i32 = (start_row as i32) + (index as i32) * dx;
            let y: i32 = (star_col as i32) + (index as i32) * dy;

            is_in_bound(grid, x, y) && grid[x as usize][y as usize] == ch
        })
}

fn nb_occurences(grid: &Vec<Vec<char>>) -> usize {
    let directions: [(i32, i32); 8] = [
        (0, 1), // Horizontal
        (1, 0), // Vertical
        (1, 1), // Diagonal down-right
        (-1, 1), // Diagonal up-right
        (0, -1), // Horizontal backwards
        (-1, 0), // Vertical upwards
        (-1, -1), // Diagonal up-left
        (1, -1), // Diagonal down-left
    ];

    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let mut count: usize = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                if is_word_at_in_direction(&grid, XMAS, row, col, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let grid: Vec<Vec<char>> = INPUT.lines()
        .map(|line| line.chars().collect())
        .collect();

    let nb_xmas: usize = nb_occurences(&grid);

    println!("Number of XMAS: {}", nb_xmas);
}
