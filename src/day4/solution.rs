use std::fs;

const INPUT: &str = "src/day4/input.txt";

// static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
#[allow(dead_code)]
pub fn part1() {
    let content = fs::read_to_string(INPUT).unwrap();
    let m = content.lines().count();
    let n = content.lines().into_iter().count();
    let mut grid = vec![vec![' '; n]; m];
    for (i, line) in content.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[i][j] = ch;
        }
    }

    // println!("{:?}", grid);

    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 'X' {
                continue;
            }
            res += count_xmas(&grid, i, j);
        }
    }

    println!("day4 part1 solution: {res}");
}

fn count_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let m = grid.len();
    let n = grid.first().unwrap().len();

    let mut res = 0;

    // up
    if i >= 3
        && grid[i][j] == 'X'
        && grid[i - 1][j] == 'M'
        && grid[i - 2][j] == 'A'
        && grid[i - 3][j] == 'S'
    {
        res += 1;
    }

    // right
    if j + 3 < n
        && grid[i][j] == 'X'
        && grid[i][j + 1] == 'M'
        && grid[i][j + 2] == 'A'
        && grid[i][j + 3] == 'S'
    {
        res += 1;
    }

    // down
    if i + 3 < m
        && grid[i][j] == 'X'
        && grid[i + 1][j] == 'M'
        && grid[i + 2][j] == 'A'
        && grid[i + 3][j] == 'S'
    {
        res += 1;
    }

    // left
    if j >= 3
        && grid[i][j] == 'X'
        && grid[i][j - 1] == 'M'
        && grid[i][j - 2] == 'A'
        && grid[i][j - 3] == 'S'
    {
        res += 1;
    }

    // diag
    // up left
    if i >= 3
        && j >= 3
        && grid[i][j] == 'X'
        && grid[i - 1][j - 1] == 'M'
        && grid[i - 2][j - 2] == 'A'
        && grid[i - 3][j - 3] == 'S'
    {
        res += 1;
    }

    // up right
    if i >= 3
        && j + 3 < n
        && grid[i][j] == 'X'
        && grid[i - 1][j + 1] == 'M'
        && grid[i - 2][j + 2] == 'A'
        && grid[i - 3][j + 3] == 'S'
    {
        res += 1;
    }

    // down right
    if i + 3 < m
        && j + 3 < n
        && grid[i][j] == 'X'
        && grid[i + 1][j + 1] == 'M'
        && grid[i + 2][j + 2] == 'A'
        && grid[i + 3][j + 3] == 'S'
    {
        res += 1;
    }

    // down left
    if i + 3 < m
        && j >= 3
        && grid[i][j] == 'X'
        && grid[i + 1][j - 1] == 'M'
        && grid[i + 2][j - 2] == 'A'
        && grid[i + 3][j - 3] == 'S'
    {
        res += 1;
    }
    res
}

#[allow(dead_code)]
pub fn part2() {
    let content = fs::read_to_string(INPUT).unwrap();
    let m = content.lines().count();
    let n = content.lines().into_iter().count();
    let mut grid = vec![vec![' '; n]; m];
    for (i, line) in content.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[i][j] = ch;
        }
    }

    // println!("{:?}", grid);

    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 'M' {
                continue;
            }
            res += count_x_mas(&grid, i, j);
        }
    }
    // each block is counted twice
    res = res / 2;

    println!("day4 part2 solution: {res}");
}

enum Dir {
    TopLeft,
    TopRight,
    DownLeft,
    DownRight,
}

fn has_max(grid: &Vec<Vec<char>>, i: usize, j: usize, dir: Dir) -> bool {
    let m = grid.len();
    let n = grid.first().unwrap().len();
    match dir {
        Dir::TopLeft => {
            if i >= 2
                && j >= 2
                && grid[i][j] == 'M'
                && grid[i - 1][j - 1] == 'A'
                && grid[i - 2][j - 2] == 'S'
            {
                return true;
            }
            return false;
        }
        Dir::TopRight => {
            if i >= 2
                && j + 2 < n
                && grid[i][j] == 'M'
                && grid[i - 1][j + 1] == 'A'
                && grid[i - 2][j + 2] == 'S'
            {
                return true;
            }
            return false;
        }
        Dir::DownLeft => {
            if i + 2 < m
                && j >= 2
                && grid[i][j] == 'M'
                && grid[i + 1][j - 1] == 'A'
                && grid[i + 2][j - 2] == 'S'
            {
                return true;
            }
            return false;
        }
        Dir::DownRight => {
            if i + 2 < m
                && j + 2 < n
                && grid[i][j] == 'M'
                && grid[i + 1][j + 1] == 'A'
                && grid[i + 2][j + 2] == 'S'
            {
                return true;
            }
            return false;
        }
    }
}

fn count_x_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    // for each M, we can check four directions
    // and each of the four directions have two directions to check
    // 8 cases in total

    let mut res = 0;

    // up left
    if has_max(grid, i, j, Dir::TopLeft) {
        if has_max(grid, i - 2, j, Dir::DownLeft) || has_max(grid, i, j - 2, Dir::TopRight) {
            res += 1;
        }
    }

    // up right
    if has_max(grid, i, j, Dir::TopRight) {
        if has_max(grid, i, j + 2, Dir::TopLeft) || has_max(grid, i - 2, j, Dir::DownRight) {
            res += 1;
        }
    }

    // down left
    if has_max(grid, i, j, Dir::DownLeft) {
        if has_max(grid, i + 2, j, Dir::TopLeft) || has_max(grid, i, j - 2, Dir::DownRight) {
            res += 1;
        }
    }

    // down right
    if has_max(grid, i, j, Dir::DownRight) {
        if has_max(grid, i + 2, j, Dir::TopRight) || has_max(grid, i, j + 2, Dir::DownLeft) {
            res += 1;
        }
    }

    res
}

// fn has_xmas(res: &mut i32, grid: &Vec<Vec<char>>, i: usize, j: usize, idx: usize) -> bool {
//     if idx == XMAS.len() {
//         return true;
//     }

//     let m = grid.len();
//     let n = grid.first().iter().len();
//     if i >= m || j >= n {
//         return false;
//     }
//     if grid[i][j] != XMAS[idx] {
//         return false;
//     }

//     // up
//     if has_xmas(res, grid, i - 1, j, idx + 1) {
//         *res += 1
//     }
//     // right
//     if has_xmas(res, grid, i, j + 1, idx + 1) {
//         *res += 1
//     }
//     // down
//     if has_xmas(res, grid, i + 1, j, idx + 1) {
//         *res += 1
//     }
//     // left
//     if has_xmas(res, grid, i, j - 1, idx + 1) {
//         *res += 1
//     }

//     // diag
//     // up left
//     if has_xmas(res, grid, i - 1, j - 1, idx + 1) {
//         *res += 1
//     }
//     // up right
//     // bottom right
//     // bottom left

//     true
// }
