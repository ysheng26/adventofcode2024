use std::{collections::HashSet, fs};

const INPUT: &str = "src/day10/input.txt";
// const INPUT: &str = "src/day10/input-small.txt";
// const INPUT: &str = "src/day10/input-medium.txt";

#[derive(Debug)]
struct Grid(Vec<Vec<i32>>);
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
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = fs::read_to_string(input)
            .unwrap()
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|ch| ch.is_digit(10))
                    .map(|ch| ch.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
        Self(grid)
    }

    fn get_starting_pos(&self) -> Vec<Pos> {
        let m = self.0.len();
        let n = self.0[0].len();

        let mut res = Vec::<Pos>::new();

        for i in 0..m {
            for j in 0..n {
                if self.0[i][j] == 0 {
                    res.push((i, j))
                }
            }
        }
        res
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

    fn get(&self, pos: Pos) -> i32 {
        self.0[pos.0][pos.1]
    }

    fn solve_part1(&self, visited: &mut HashSet<Pos>, pos: Pos) -> i32 {
        // let m = self.0.len();
        // let n = self.0[0].len();

        // // not really needed
        // if pos.0 >= m || pos.1 >= n {
        //     return 0;
        // }

        if self.get(pos) == 9 {
            return 1;
        }

        // for each direction
        let next_pos_list = vec![
            self.get_next_pos(pos, &Dir::Up),
            self.get_next_pos(pos, &Dir::Right),
            self.get_next_pos(pos, &Dir::Left),
            self.get_next_pos(pos, &Dir::Down),
        ];
        let next_pos_list = next_pos_list
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap());

        let mut res = 0;
        for next_pos in next_pos_list {
            if visited.contains(&next_pos) {
                continue;
            }
            if self.get(next_pos) - self.get(pos) != 1 {
                continue;
            }

            visited.insert(next_pos);
            res += self.solve_part1(visited, next_pos);
            // no need to remove
            // visited.remove(&next_pos);
        }

        return res;
    }

    fn solve_part2(&self, visited: &mut HashSet<Pos>, pos: Pos) -> i32 {
        // let m = self.0.len();
        // let n = self.0[0].len();

        // // not really needed
        // if pos.0 >= m || pos.1 >= n {
        //     return 0;
        // }

        if self.get(pos) == 9 {
            return 1;
        }

        // for each direction
        let next_pos_list = vec![
            self.get_next_pos(pos, &Dir::Up),
            self.get_next_pos(pos, &Dir::Right),
            self.get_next_pos(pos, &Dir::Left),
            self.get_next_pos(pos, &Dir::Down),
        ];
        let next_pos_list = next_pos_list
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap());

        let mut res = 0;
        for next_pos in next_pos_list {
            if visited.contains(&next_pos) {
                continue;
            }
            if self.get(next_pos) - self.get(pos) != 1 {
                continue;
            }

            visited.insert(next_pos);
            res += self.solve_part2(visited, next_pos);
            visited.remove(&next_pos);
        }

        return res;
    }
}

#[allow(dead_code)]
pub fn part1() {
    let grid = Grid::new(INPUT);
    // println!("{:?}", grid);

    let starting_pos = grid.get_starting_pos();
    // println!("{:?}", starting_pos);

    let mut res = 0;
    for pos in starting_pos {
        let mut visited = HashSet::<Pos>::new();
        res += grid.solve_part1(&mut visited, pos);
    }

    println!("day10 part1 solution: {res}");
}

#[allow(dead_code)]
pub fn part2() {
    let grid = Grid::new(INPUT);
    // println!("{:?}", grid);

    let starting_pos = grid.get_starting_pos();
    // println!("{:?}", starting_pos);

    let mut res = 0;
    for pos in starting_pos {
        let mut visited = HashSet::<Pos>::new();
        res += grid.solve_part2(&mut visited, pos);
    }

    println!("day10 part2 solution: {res}");
}

/*
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....

 */
