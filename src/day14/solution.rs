use std::{collections::HashSet, fs};

use regex::Regex;

const INPUT: &str = "src/day14/input.txt";
// const INPUT: &str = "src/day14/input-small.txt";

// this is how many rows, it's the y-axis
const M: i32 = 103;
// const M: i32 = 7;

// this is how many columns, it's the x-axis
const N: i32 = 101;
// const N: i32 = 11;

type Vector2 = (i32, i32);

#[derive(Debug)]
struct Robot {
    position: Vector2,
    velocity: Vector2,
    quad_id: Option<usize>,
}

impl Robot {
    fn new(pos_x: i32, pos_y: i32, speed_x: i32, speed_y: i32) -> Self {
        //
        Robot {
            position: (pos_x, pos_y),
            velocity: (speed_x, speed_y),
            quad_id: None,
        }
    }
    fn step(&mut self, seconds: i32) {
        let mut x = self.position.0 + seconds * self.velocity.0;
        let mut y = self.position.1 + seconds * self.velocity.1;

        x = (x % N + N) % N;
        // x = x.rem_euclid(M);
        y = (y % M + M) % M;
        // y = y.rem_euclid(N);

        self.position = (x, y);
    }

    fn quadrant(&self) -> Option<usize> {
        let mid_x = N / 2;
        let mid_y = M / 2;

        let x = self.position.0;
        let y = self.position.1;
        if x < mid_x && y < mid_y {
            return Some(0);
        }

        if x > mid_x && y < mid_y {
            return Some(1);
        }

        if x > mid_x && y > mid_y {
            return Some(2);
        }

        if x < mid_x && y > mid_y {
            return Some(3);
        }

        None
    }
}

#[allow(unused)]
pub fn part1() {
    let mut robots = Vec::<Robot>::new();

    let re = Regex::new(r"=(-?\d+),(-?\d+).*=(-?\d+),(-?\d+)").unwrap();
    for line in fs::read_to_string(INPUT).unwrap().lines() {
        for (_, [pos_x, pos_y, speed_x, speed_y]) in re.captures_iter(&line).map(|c| c.extract()) {
            let pos_x = pos_x.parse::<i32>().unwrap();
            let pos_y = pos_y.parse::<i32>().unwrap();
            let speed_x = speed_x.parse::<i32>().unwrap();
            let speed_y = speed_y.parse::<i32>().unwrap();
            robots.push(Robot::new(pos_x, pos_y, speed_x, speed_y));
        }
    }

    let mut counts = vec![0, 0, 0, 0];
    for robot in &mut robots {
        robot.step(100);

        if let Some(quad_id) = robot.quadrant() {
            robot.quad_id = Some(quad_id);
            counts[quad_id] += 1;
        }
    }
    println!("{:?}", counts);
    let res: i64 = counts.iter().fold(1, |acc, x| acc * x);
    println!("day14 part1 solution: {res}");
}

fn is_christmas_tree(robots: &[Robot]) -> bool {
    //
    let hashset: HashSet<(i32, i32)> = robots.iter().map(|robot| robot.position).collect();
    hashset.len() == robots.len()
}

#[allow(unused)]
pub fn part2() {
    let mut robots = Vec::<Robot>::new();

    let re = Regex::new(r"=(-?\d+),(-?\d+).*=(-?\d+),(-?\d+)").unwrap();
    for line in fs::read_to_string(INPUT).unwrap().lines() {
        for (_, [pos_x, pos_y, speed_x, speed_y]) in re.captures_iter(&line).map(|c| c.extract()) {
            let pos_x = pos_x.parse::<i32>().unwrap();
            let pos_y = pos_y.parse::<i32>().unwrap();
            let speed_x = speed_x.parse::<i32>().unwrap();
            let speed_y = speed_y.parse::<i32>().unwrap();
            robots.push(Robot::new(pos_x, pos_y, speed_x, speed_y));
        }
    }
    let mut res = 0;
    // it was the 2nd answer
    loop {
        if is_christmas_tree(&robots) {
            println!("{res}");
        }
        res += 1;
        for robot in &mut robots {
            robot.step(1);
        }
    }
    println!("day14 part2 solution: {res}");
}
