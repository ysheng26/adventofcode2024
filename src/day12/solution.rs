use std::{collections::HashSet, fs};

// const INPUT: &str = "src/day12/input-small.txt";
// const INPUT: &str = "src/day12/input-medium.txt";
const INPUT: &str = "src/day12/input.txt";

#[derive(Debug)]
struct Grid(Vec<Vec<char>>);
type Pos = (usize, usize);

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
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

// TODO: Grid<T>
impl Grid {
    fn new(input: &str) -> Self {
        let grid = fs::read_to_string(input)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self(grid)
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
            Dir::UpLeft => match (check_add(pos.0, -1), check_add(pos.1, -1)) {
                (Some(nx), Some(ny)) => {
                    if nx < m && ny < n {
                        return Some((nx, ny));
                    }
                    None
                }
                (_, _) => None,
            },
            Dir::UpRight => match (check_add(pos.0, -1), check_add(pos.1, 1)) {
                (Some(nx), Some(ny)) => {
                    if nx < m && ny < n {
                        return Some((nx, ny));
                    }
                    None
                }
                (_, _) => None,
            },
            Dir::DownLeft => match (check_add(pos.0, 1), check_add(pos.1, -1)) {
                (Some(nx), Some(ny)) => {
                    if nx < m && ny < n {
                        return Some((nx, ny));
                    }
                    None
                }
                (_, _) => None,
            },
            Dir::DownRight => match (check_add(pos.0, 1), check_add(pos.1, 1)) {
                (Some(nx), Some(ny)) => {
                    if nx < m && ny < n {
                        return Some((nx, ny));
                    }
                    None
                }
                (_, _) => None,
            },
        }
    }

    fn get(&self, pos: Pos) -> char {
        self.0[pos.0][pos.1]
    }

    fn get_size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn fill_area_and_perimeter(
        &self,
        pos: Pos,
        visited: &mut HashSet<Pos>,
        ch: char,
        area: &mut i32,
        perimeter: &mut i32,
    ) {
        *area += 1;
        let mut same_neighbors = 0;

        let dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        for dir in dirs {
            if let Some(next_pos) = self.get_next_pos(pos, &dir) {
                if self.get(next_pos) == ch {
                    same_neighbors += 1;
                }

                if visited.contains(&next_pos) {
                    continue;
                }
                if self.get(next_pos) != ch {
                    continue;
                }
                visited.insert(next_pos);
                self.fill_area_and_perimeter(next_pos, visited, ch, area, perimeter);
            }
        }
        *perimeter += 4 - same_neighbors;
        // println!(
        //     "same_neighbors at pos ({}, {}), {}",
        //     pos.0, pos.1, same_neighbors
        // )
    }

    fn fill_area_and_verticies(
        &self,
        pos: Pos,
        visited: &mut HashSet<Pos>,
        ch: char,
        area: &mut i32,
        vertices: &mut i32,
    ) {
        *area += 1;

        // check top left corner
        //    xx
        //    xa
        //
        //    xa
        //    aa
        //
        //    xa
        //    ax
        //
        // if top left is None or different
        //      if (left is non or diff) and (top is non or different)
        //          v++
        //      if left and top are same
        //          v++
        // else if top left is same char
        //      if left top is diff and left is diff
        //          v++

        let up = self.get_next_pos(pos, &Dir::Up);
        let upleft = self.get_next_pos(pos, &Dir::UpLeft);
        let left = self.get_next_pos(pos, &Dir::Left);
        if upleft.is_none() || self.get(upleft.unwrap()) != ch {
            if (left.is_none() || self.get(left.unwrap()) != ch)
                && (up.is_none() || self.get(up.unwrap()) != ch)
            {
                *vertices += 1;
            } else if left.is_some()
                && self.get(left.unwrap()) == ch
                && up.is_some()
                && self.get(up.unwrap()) == ch
            {
                *vertices += 1;
            }
        } else if upleft.is_some() && self.get(upleft.unwrap()) == ch {
            if left.is_some()
                && self.get(left.unwrap()) != ch
                && up.is_some()
                && self.get(up.unwrap()) != ch
            {
                *vertices += 1;
            }
        }

        // check top right corner
        let up = self.get_next_pos(pos, &Dir::Up);
        let upright = self.get_next_pos(pos, &Dir::UpRight);
        let right = self.get_next_pos(pos, &Dir::Right);
        if upright.is_none() || self.get(upright.unwrap()) != ch {
            if (right.is_none() || self.get(right.unwrap()) != ch)
                && (up.is_none() || self.get(up.unwrap()) != ch)
            {
                *vertices += 1;
            } else if right.is_some()
                && self.get(right.unwrap()) == ch
                && up.is_some()
                && self.get(up.unwrap()) == ch
            {
                *vertices += 1;
            }
        } else if upright.is_some() && self.get(upright.unwrap()) == ch {
            if right.is_some()
                && self.get(right.unwrap()) != ch
                && up.is_some()
                && self.get(up.unwrap()) != ch
            {
                *vertices += 1;
            }
        }

        // check bottom left corner
        let down = self.get_next_pos(pos, &Dir::Down);
        let downleft = self.get_next_pos(pos, &Dir::DownLeft);
        let left = self.get_next_pos(pos, &Dir::Left);
        if downleft.is_none() || self.get(downleft.unwrap()) != ch {
            if (left.is_none() || self.get(left.unwrap()) != ch)
                && (down.is_none() || self.get(down.unwrap()) != ch)
            {
                *vertices += 1;
            } else if left.is_some()
                && self.get(left.unwrap()) == ch
                && down.is_some()
                && self.get(down.unwrap()) == ch
            {
                *vertices += 1;
            }
        } else if downleft.is_some() && self.get(downleft.unwrap()) == ch {
            if left.is_some()
                && self.get(left.unwrap()) != ch
                && down.is_some()
                && self.get(down.unwrap()) != ch
            {
                *vertices += 1;
            }
        }

        // check bottom right corner
        let down = self.get_next_pos(pos, &Dir::Down);
        let downright = self.get_next_pos(pos, &Dir::DownRight);
        let right = self.get_next_pos(pos, &Dir::Right);
        if downright.is_none() || self.get(downright.unwrap()) != ch {
            if (right.is_none() || self.get(right.unwrap()) != ch)
                && (down.is_none() || self.get(down.unwrap()) != ch)
            {
                *vertices += 1;
            } else if right.is_some()
                && self.get(right.unwrap()) == ch
                && down.is_some()
                && self.get(down.unwrap()) == ch
            {
                *vertices += 1;
            }
        } else if downright.is_some() && self.get(downright.unwrap()) == ch {
            if right.is_some()
                && self.get(right.unwrap()) != ch
                && down.is_some()
                && self.get(down.unwrap()) != ch
            {
                *vertices += 1;
            }
        }

        let dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        for dir in dirs {
            if let Some(next_pos) = self.get_next_pos(pos, &dir) {
                if visited.contains(&next_pos) {
                    continue;
                }
                if self.get(next_pos) != ch {
                    continue;
                }
                visited.insert(next_pos);
                self.fill_area_and_verticies(next_pos, visited, ch, area, vertices);
            }
        }
    }

    fn solve_part1(&self, visited: &mut HashSet<Pos>) -> i32 {
        let (m, n) = self.get_size();
        let mut res = 0;

        for i in 0..m {
            for j in 0..n {
                let pos = (i, j);
                if visited.contains(&pos) {
                    continue;
                }

                let mut area = 0;
                let mut perimeter = 0;
                visited.insert(pos);
                self.fill_area_and_perimeter(
                    pos,
                    visited,
                    self.get(pos),
                    &mut area,
                    &mut perimeter,
                );
                // println!(
                //     "Pos: ({}, {}), ch: {}, area = {}, perimeter = {}",
                //     pos.0,
                //     pos.1,
                //     self.get(pos),
                //     area,
                //     perimeter
                // );
                res += area * perimeter;
            }
        }
        res
    }

    fn solve_part2(&self, visited: &mut HashSet<Pos>) -> i32 {
        let (m, n) = self.get_size();
        let mut res = 0;

        for i in 0..m {
            for j in 0..n {
                let pos = (i, j);
                if visited.contains(&pos) {
                    continue;
                }

                let mut area = 0;
                let mut verticies = 0;
                visited.insert(pos);
                self.fill_area_and_verticies(
                    pos,
                    visited,
                    self.get(pos),
                    &mut area,
                    &mut verticies,
                );
                res += area * verticies;
            }
        }
        res
    }
}

#[allow(dead_code)]
pub fn part1() {
    let grid = Grid::new(INPUT);
    // println!("{:?}", grid);

    // for the perimeter, count how many same neighbors are there
    // then use 4 - that number

    let mut visited = HashSet::<Pos>::new();
    let res = grid.solve_part1(&mut visited);
    println!("day12 part1 solution: {res}");
}

#[allow(dead_code)]
pub fn part2() {
    let grid = Grid::new(INPUT);

    let mut visited = HashSet::<Pos>::new();
    let res = grid.solve_part2(&mut visited);
    println!("day12 part2 solution: {res}");
}
