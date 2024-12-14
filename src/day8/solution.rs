use std::{collections::HashSet, fs};

use itertools::Itertools;

// const INPUT: &str = "src/day8/input-small.txt";
const INPUT: &str = "src/day8/input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Vec2 {
    fn from(value: (i32, i32)) -> Self {
        Vec2::new(value.0, value.1)
    }
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }

    fn to(&self, v: &Vec2) -> Self {
        Vec2 {
            x: v.x - self.x,
            y: v.y - self.y,
        }
    }

    fn add(&self, v: &Vec2) -> Self {
        Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    fn scale(&self, n: i32) -> Self {
        Vec2 {
            x: self.x * n,
            y: self.y * n,
        }
    }

    fn invert(&self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }

    fn in_bounds(&self, m: i32, n: i32) -> bool {
        if self.x >= 0 && self.x < m && self.y >= 0 && self.y < n {
            return true;
        }
        false
    }
}

fn handle_coordinates(seen: &mut HashSet<Vec2>, xs: &[(i32, i32)], m: i32, n: i32) {
    for i in 0..xs.len() {
        for j in i..xs.len() {
            if i == j {
                continue;
            }

            let a: Vec2 = xs[i].into();
            let b: Vec2 = xs[j].into();

            let c = a.to(&b);
            let bb = b.add(&c);
            if bb.in_bounds(m, n) {
                seen.insert(bb);
            }

            let c = b.to(&a);
            let aa = a.add(&c);
            if aa.in_bounds(m, n) {
                seen.insert(aa);
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1() {
    let m = fs::read_to_string(INPUT).unwrap().lines().count();
    let mut n = 0;

    let mut tmp = Vec::<(char, (i32, i32))>::new();
    for (i, line) in fs::read_to_string(INPUT).unwrap().lines().enumerate() {
        n = line.len();
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            tmp.push((c, (i as i32, j as i32)));
        }
    }

    let input = tmp.into_iter().into_group_map();

    let mut seen = HashSet::<Vec2>::new();
    for (_, xs) in input {
        // println!("working on {ch}");
        handle_coordinates(&mut seen, &xs, m as i32, n as i32);
    }
    let res = seen.len();
    println!("day8 part1 solution: {res}");
}

fn handle_coordinates2(seen: &mut HashSet<Vec2>, xs: &[(i32, i32)], m: i32, n: i32) {
    for i in 0..xs.len() {
        for j in i..xs.len() {
            if i == j {
                continue;
            }

            let a: Vec2 = xs[i].into();
            let b: Vec2 = xs[j].into();

            // too lazy to do it the right way
            let v = a.to(&b);
            for i in 0..200 {
                let bb = b.add(&v.scale(i));
                if bb.in_bounds(m, n) {
                    seen.insert(bb);
                }

                let aa = a.add(&v.invert().scale(i));
                if aa.in_bounds(m, n) {
                    seen.insert(aa);
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn part2() {
    let m = fs::read_to_string(INPUT).unwrap().lines().count();
    let mut n = 0;

    let mut tmp = Vec::<(char, (i32, i32))>::new();
    for (i, line) in fs::read_to_string(INPUT).unwrap().lines().enumerate() {
        n = line.len();
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            tmp.push((c, (i as i32, j as i32)));
        }
    }

    let input = tmp.into_iter().into_group_map();

    let mut seen = HashSet::<Vec2>::new();
    for (_, xs) in input {
        // println!("working on {ch}");
        handle_coordinates2(&mut seen, &xs, m as i32, n as i32);
    }
    let res = seen.len();
    println!("day8 part2 solution: {res}");
}
