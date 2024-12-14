use std::{collections::HashSet, fs};

const INPUT: &str = "src/day6/input.txt";
// const INPUT: &str = "src/day6/input-small.txt";

type Pos = (usize, usize);

struct Grid(Vec<Vec<char>>);

fn check_add(a: usize, b: i32) -> Option<usize> {
    // if b is negative and abs > a
    // then result would be invalid
    if b < 0 && b.wrapping_abs() as usize > a {
        return None;
    }

    if b < 0 {
        return Some(a - b.wrapping_abs() as usize);
    }
    Some(a + b as usize)
}

enum Dir {
    Up,
    Left,
    Right,
    Down,
}

impl Dir {
    fn rotate_90(&self) -> Dir {
        match &self {
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
        }
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = fs::read_to_string(input)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self(grid)
    }

    fn get_starting_pos(&self) -> Pos {
        let m = self.0.len();
        let n = self.0[0].len();

        let mut start: Pos = (0, 0);
        for i in 0..m {
            for j in 0..n {
                if self.0[i][j] == '^' {
                    start = (i, j);
                }
            }
        }
        start
    }

    fn get_next_pos(&self, pos: Pos, dir: &Dir) -> Option<Pos> {
        let m = self.0.len();
        let n = self.0[0].len();

        match dir {
            Dir::Up => {
                // let nx = pos.0 - 1;
                let nx = check_add(pos.0, -1);
                let ny = pos.1;
                if let Some(nx) = nx {
                    if nx < m && ny < n {
                        return Some((nx, ny));
                    }
                }

                None
            }
            Dir::Left => {
                let nx = pos.0;
                // let ny = pos.1 - 1;
                let ny = check_add(pos.1, -1);
                if let Some(ny) = ny {
                    if nx < m && ny < n {
                        return Some((nx, ny));
                    }
                }

                None
            }
            Dir::Right => {
                let nx = pos.0;
                let ny = pos.1 + 1;
                if nx < m && ny < n {
                    return Some((nx, ny));
                }
                None
            }
            Dir::Down => {
                let nx = pos.0 + 1;
                let ny = pos.1;
                if nx < m && ny < n {
                    return Some((nx, ny));
                }
                None
            }
        }
    }

    fn solve_part1(&self, visited: &mut HashSet<Pos>, pos: Pos, dir: Dir) {
        // keep moving to the direction until hit a # or if out of bounds
        // then stop. Might need a bool to indicate that, we'll see

        if let Some(next_pos) = self.get_next_pos(pos, &dir) {
            // hit a block
            if self.0[next_pos.0][next_pos.1] == '#' {
                self.solve_part1(visited, pos, dir.rotate_90());
            } else {
                visited.insert(next_pos);
                self.solve_part1(visited, next_pos, dir);
            }
        } else {
            // out of bounds
            return;
        }
    }

    fn solve_part2(
        &self,
        visited: &mut HashSet<Pos>,
        pos: Pos,
        dir: Dir,
        mut repeated: i32,
    ) -> bool {
        // keep moving to the direction until hit a # or if out of bounds
        // then stop. Might need a bool to indicate that, we'll see

        // don't know how to tell if he is stuck
        // a lot of consecutive visited?
        if repeated >= 1000 {
            return true;
        }

        if let Some(next_pos) = self.get_next_pos(pos, &dir) {
            // hit a block
            if self.0[next_pos.0][next_pos.1] == '#' {
                return self.solve_part2(visited, pos, dir.rotate_90(), repeated);
            } else {
                if visited.contains(&next_pos) {
                    repeated += 1;
                }
                visited.insert(next_pos);
                return self.solve_part2(visited, next_pos, dir, repeated);
            }
        } else {
            // out of bounds
            // nah he went out
            return false;
        }
    }
}

#[allow(dead_code)]
pub fn part1() {
    let grid = Grid::new(INPUT);
    let start_pos = grid.get_starting_pos();

    let mut visited = HashSet::<Pos>::new();
    visited.insert(start_pos);

    grid.solve_part1(&mut visited, start_pos, Dir::Up);

    let res = visited.len();
    println!("day6 part1 solution: {res}");
}

#[allow(dead_code)]
pub fn part2() {
    let mut grid = Grid::new(INPUT);
    let start_pos = grid.get_starting_pos();

    let mut res = 0;

    let m = grid.0.len();
    let n = grid.0[0].len();
    for i in 0..m {
        for j in 0..n {
            // can not block starting position
            if start_pos == (i, j) {
                continue;
            }

            // already a block, skip
            if grid.0[i][j] == '#' {
                continue;
            }

            let mut visited = HashSet::<Pos>::new();
            visited.insert(start_pos);
            grid.0[i][j] = '#';
            if grid.solve_part2(&mut visited, start_pos, Dir::Up, 0) {
                res += 1;
            }
            grid.0[i][j] = '.';
        }
    }

    println!("day6 part2 solution: {res}");
}
