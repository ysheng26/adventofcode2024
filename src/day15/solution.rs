use std::fs;

// const INPUT: &str = "src/day15/input-medium.txt";
const INPUT: &str = "src/day15/input.txt";
// const INPUT: &str = "src/day15/input-small.txt";

// 100 * i + j

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    m: usize,
    n: usize,
    robot_pos: Pos,
}

type Pos = (usize, usize);

fn check_add(a: usize, b: i32) -> Option<usize> {
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
    Down,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Dir::Up),
            '>' => Ok(Dir::Right),
            '<' => Ok(Dir::Left),
            'v' => Ok(Dir::Down),
            _ => Err("Invalid direction"),
        }
    }
}

const WALL: char = '#';
const BOX: char = 'O';
const BOT: char = '@';
const SPACE: char = '.';

const BIGBOX_LEFT: char = '[';
const BIGBOX_RIGHT: char = ']';

impl Grid {
    fn new2(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|ch| match ch {
                        WALL => "##".chars(),
                        BOX => "[]".chars(),
                        BOT => "@.".chars(),
                        SPACE => "..".chars(),
                        _ => "  ".chars(),
                    })
                    .collect()
            })
            .collect();
        let m = grid.len();
        let n = grid[0].len();

        let mut robot_pos = (0, 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == BOT {
                    robot_pos = (i, j);
                    break;
                }
            }
        }

        Self {
            grid,
            m,
            n,
            robot_pos,
        }
    }

    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let m = grid.len();
        let n = grid[0].len();

        let mut robot_pos = (0, 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == BOT {
                    robot_pos = (i, j);
                    break;
                }
            }
        }

        Self {
            grid,
            m,
            n,
            robot_pos,
        }
    }

    // fn get(&self, pos: &Pos) -> Option<char> {
    //     //
    //     None
    // }

    fn move_robot_right(&mut self) {
        // from robot pos to right
        // find if there are any '.' spaces
        // if so break and move everything from robot to right once

        // j++ until we see a '.' or we are at the wall
        let i = self.robot_pos.0;
        let mut j = self.robot_pos.1;
        while self.grid[i][j] != SPACE && self.grid[i][j] != WALL {
            j += 1;
        }

        if self.grid[i][j] == WALL {
            return;
        }

        // 0 1 2 3 4 5
        //   @ . O #
        //         j
        //       k
        //
        // 0 1 2 3 4 5
        //     @ . O #
        //       j
        //       k
        // we actually have space, move everything from robot_pos to right once
        // for k in (self.robot_pos.1 + 1..j).rev() {
        for k in (self.robot_pos.1 + 1..=j).rev() {
            self.grid[i][k] = self.grid[i][k - 1];
            self.grid[i][k - 1] = SPACE;
        }
        self.robot_pos.1 += 1;
    }

    fn move_robot_up(&mut self) {
        let mut i = self.robot_pos.0;
        let j = self.robot_pos.1;
        while self.grid[i][j] != SPACE && self.grid[i][j] != WALL {
            i -= 1;
        }

        if self.grid[i][j] == WALL {
            return;
        }

        for k in i..self.robot_pos.0 {
            self.grid[k][j] = self.grid[k + 1][j];
            self.grid[k + 1][j] = SPACE;
        }
        self.robot_pos.0 -= 1;
    }

    fn can_move_robot_up2(&mut self, pos: Pos) -> bool {
        let x = pos.0;
        let y = pos.1;
        let curr_ch = self.grid[x][y];
        if curr_ch == WALL {
            return false;
        }

        if curr_ch == SPACE {
            return true;
        }

        if curr_ch == BOT {
            if self.can_move_robot_up2((x - 1, y)) {
                return true;
            }
        }

        if curr_ch == BIGBOX_LEFT {
            let a = self.can_move_robot_up2((x - 1, y));
            let b = self.can_move_robot_up2((x - 1, y + 1));
            return a && b;
        }

        if curr_ch == BIGBOX_RIGHT {
            //
            let a = self.can_move_robot_up2((x - 1, y));
            let b = self.can_move_robot_up2((x - 1, y - 1));
            return a && b;
        }

        false
    }

    fn move_robot_up2(&mut self, pos: Pos) -> bool {
        let x = pos.0;
        let y = pos.1;
        let curr_ch = self.grid[x][y];
        if curr_ch == WALL {
            return false;
        }

        if curr_ch == SPACE {
            return true;
        }

        if curr_ch == BOT {
            if self.can_move_robot_up2((x - 1, y)) {
                self.move_robot_up2((x - 1, y));
                self.grid[x][y] = SPACE;
                self.grid[x - 1][y] = BOT;
                self.robot_pos = (x - 1, y);
            }
        }

        if curr_ch == BIGBOX_LEFT {
            let a = self.can_move_robot_up2((x - 1, y));
            let b = self.can_move_robot_up2((x - 1, y + 1));
            if a && b {
                self.move_robot_up2((x - 1, y));
                self.move_robot_up2((x - 1, y + 1));
                self.grid[x][y] = SPACE;
                self.grid[x][y + 1] = SPACE;
                self.grid[x - 1][y] = BIGBOX_LEFT;
                self.grid[x - 1][y + 1] = BIGBOX_RIGHT;
                return true;
            }
        }

        if curr_ch == BIGBOX_RIGHT {
            //
            let a = self.can_move_robot_up2((x - 1, y));
            let b = self.can_move_robot_up2((x - 1, y - 1));
            if a && b {
                self.move_robot_up2((x - 1, y));
                self.move_robot_up2((x - 1, y - 1));
                self.grid[x][y] = SPACE;
                self.grid[x][y - 1] = SPACE;
                self.grid[x - 1][y] = BIGBOX_RIGHT;
                self.grid[x - 1][y - 1] = BIGBOX_LEFT;
                return true;
            }
        }

        false
    }

    fn move_robot_down(&mut self) {
        //
        let mut i = self.robot_pos.0;
        let j = self.robot_pos.1;
        while self.grid[i][j] != SPACE && self.grid[i][j] != WALL {
            i += 1;
        }

        if self.grid[i][j] == WALL {
            return;
        }

        for k in (self.robot_pos.0 + 1..=i).rev() {
            self.grid[k][j] = self.grid[k - 1][j];
            self.grid[k - 1][j] = SPACE;
        }
        self.robot_pos.0 += 1;
    }

    fn can_move_robot_down2(&mut self, pos: Pos) -> bool {
        let x = pos.0;
        let y = pos.1;
        let curr_ch = self.grid[x][y];
        if curr_ch == WALL {
            return false;
        }

        if curr_ch == SPACE {
            return true;
        }

        if curr_ch == BOT {
            if self.can_move_robot_down2((x + 1, y)) {
                return true;
            }
        }

        if curr_ch == BIGBOX_LEFT {
            let a = self.can_move_robot_down2((x + 1, y));
            let b = self.can_move_robot_down2((x + 1, y + 1));
            return a && b;
        }

        if curr_ch == BIGBOX_RIGHT {
            //
            let a = self.can_move_robot_down2((x + 1, y));
            let b = self.can_move_robot_down2((x + 1, y - 1));
            return a && b;
        }

        false
    }

    fn move_robot_down2(&mut self, pos: Pos) -> bool {
        let x = pos.0;
        let y = pos.1;
        let curr_ch = self.grid[x][y];
        if curr_ch == WALL {
            return false;
        }

        if curr_ch == SPACE {
            return true;
        }

        if curr_ch == BOT {
            if self.can_move_robot_down2((x + 1, y)) {
                self.move_robot_down2((x + 1, y));
                self.grid[x][y] = SPACE;
                self.grid[x + 1][y] = BOT;
                self.robot_pos = (x + 1, y);
            }
        }

        if curr_ch == BIGBOX_LEFT {
            let a = self.can_move_robot_down2((x + 1, y));
            let b = self.can_move_robot_down2((x + 1, y + 1));
            if a && b {
                self.move_robot_down2((x + 1, y));
                self.move_robot_down2((x + 1, y + 1));
                self.grid[x][y] = SPACE;
                self.grid[x][y + 1] = SPACE;
                self.grid[x + 1][y] = BIGBOX_LEFT;
                self.grid[x + 1][y + 1] = BIGBOX_RIGHT;
                return true;
            }
        }

        if curr_ch == BIGBOX_RIGHT {
            //
            let a = self.can_move_robot_down2((x + 1, y));
            let b = self.can_move_robot_down2((x + 1, y - 1));
            if a && b {
                self.move_robot_down2((x + 1, y));
                self.move_robot_down2((x + 1, y - 1));
                self.grid[x][y] = SPACE;
                self.grid[x][y - 1] = SPACE;
                self.grid[x + 1][y] = BIGBOX_RIGHT;
                self.grid[x + 1][y - 1] = BIGBOX_LEFT;
                return true;
            }
        }

        false
    }

    fn move_robot_left(&mut self) {
        //
        let i = self.robot_pos.0;
        let mut j = self.robot_pos.1;
        while self.grid[i][j] != SPACE && self.grid[i][j] != WALL {
            j -= 1;
        }

        if self.grid[i][j] == WALL {
            return;
        }

        for k in j..self.robot_pos.1 {
            self.grid[i][k] = self.grid[i][k + 1];
            self.grid[i][k + 1] = SPACE;
        }
        self.robot_pos.1 -= 1;
    }

    fn move_robot_left2(&mut self, pos: Pos) -> bool {
        // need to do this recursively
        // base case is when there is space move
        // or when there is a wall then don't move

        /*
        left
         012345678
        0
        1
        2##[]@
        3##.[][]@

        move_left((2,7))
            if ch == '#' return false
            if ch == '.' return true
            if move_left((2,6))
                copy curr char one left
        */
        let x = pos.0;
        let y = pos.1;
        let curr_ch = self.grid[x][y];
        if curr_ch == WALL {
            return false;
        }

        if curr_ch == SPACE {
            return true;
        }

        if self.move_robot_left2((x, y - 1)) {
            //
            self.grid[x][y] = SPACE;
            self.grid[x][y - 1] = curr_ch;
            return true;
        }
        false
    }

    fn move_robot_right2(&mut self, pos: Pos) -> bool {
        let x = pos.0;
        let y = pos.1;
        let curr_ch = self.grid[x][y];
        if curr_ch == WALL {
            return false;
        }

        if curr_ch == SPACE {
            return true;
        }

        if self.move_robot_right2((x, y + 1)) {
            //
            self.grid[x][y] = SPACE;
            self.grid[x][y + 1] = curr_ch;
            return true;
        }
        false
    }

    fn apply_instruction(&mut self, instruction: char) {
        //
        match instruction.try_into().unwrap() {
            Dir::Up => self.move_robot_up(),
            Dir::Left => self.move_robot_left(),
            Dir::Down => self.move_robot_down(),
            Dir::Right => self.move_robot_right(),
        }
        // print_grid(&self);
    }

    fn apply_instruction2(&mut self, instruction: char) {
        //
        match instruction.try_into().unwrap() {
            // Dir::Up => self.move_robot_up2(),
            Dir::Left => {
                if self.move_robot_left2(self.robot_pos) {
                    self.robot_pos.1 -= 1;
                }
            }
            Dir::Down => {
                if self.move_robot_down2(self.robot_pos) {
                    self.robot_pos.0 += 1;
                }
            }
            Dir::Right => {
                if self.move_robot_right2(self.robot_pos) {
                    self.robot_pos.1 += 1;
                }
            }
            Dir::Up => {
                if self.move_robot_up2(self.robot_pos) {
                    self.robot_pos.0 -= 1;
                }
            }
        }
        // print_grid(&self);
    }

    fn calc_gps(&self) -> usize {
        //
        let mut res = 0;

        for i in 0..self.m {
            for j in 0..self.n {
                if self.grid[i][j] != BOX {
                    continue;
                }

                res += 100 * i + j;
            }
        }

        res
    }

    fn calc_gps2(&self) -> usize {
        //
        let mut res = 0;

        for i in 0..self.m {
            for j in 0..self.n {
                if self.grid[i][j] != BIGBOX_LEFT {
                    continue;
                }

                res += 100 * i + j;
            }
        }

        res
    }
}
fn print_grid(grid: &Grid) {
    // println!("robot_pos = {:?}", grid.robot_pos);

    for row in &grid.grid {
        for ch in row {
            print!("{ch}");
        }
        println!("");
    }
}

#[allow(unused)]
pub fn part1() {
    //
    let input: String = fs::read_to_string(INPUT).expect("can not read input file");
    let mut input_iter = input.split("\n\n");
    let grid_input = input_iter.next().unwrap();
    let instructions_input = input_iter.next().unwrap();

    let mut grid = Grid::new(&grid_input);
    let instructions: Vec<char> = instructions_input.chars().into_iter().collect();

    // print_grid(&grid);
    for instruction in instructions {
        grid.apply_instruction(instruction);
    }

    // print_grid(&grid);

    let res = grid.calc_gps();
    println!("day15 part1 solution: {res}");
}

#[allow(unused)]
pub fn part2() {
    //
    let input: String = fs::read_to_string(INPUT).expect("can not read input file");
    let mut input_iter = input.split("\n\n");
    let grid_input = input_iter.next().unwrap();
    let instructions_input = input_iter.next().unwrap();

    let mut grid = Grid::new2(&grid_input);
    let instructions: Vec<char> = instructions_input
        .chars()
        .into_iter()
        .filter(|&ch| ch == '^' || ch == '<' || ch == '>' || ch == 'v')
        .collect();
    // print_grid(&grid);
    // let mut s = String::new();
    for (i, &instruction) in instructions.iter().enumerate() {
        // println!("going to move: {instruction} {}/{}", i, instructions.len());
        // stdin()
        //     .read_line(&mut s)
        //     .expect("Did not enter a correct string");

        grid.apply_instruction2(instruction);
        // print_grid(&grid);
    }
    // print_grid(&grid);

    let res = grid.calc_gps2();
    println!("day15 part2 solution: {res}");
}
