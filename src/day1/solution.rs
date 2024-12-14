use std::{collections::HashMap, fs};

const INPUT: &str = "src/day1/input.txt";

#[allow(dead_code)]
pub fn part1() {
    let mut xs = Vec::<i32>::new();
    let mut ys = Vec::<i32>::new();

    for line in fs::read_to_string(INPUT).unwrap().lines() {
        let mut iter = line.split_ascii_whitespace();
        let x = iter.next();
        xs.push(x.unwrap().parse::<i32>().unwrap());
        let y = iter.next();
        ys.push(y.unwrap().parse::<i32>().unwrap());
    }

    xs.sort();
    ys.sort();

    let mut res = 0;
    for (i, x) in xs.iter().enumerate() {
        let curr = ys[i] - x;
        let curr = curr.abs();
        res += curr;
    }

    println!("day1 part1 solution: {res}");
}

#[allow(dead_code)]
pub fn part2() {
    let mut xs = Vec::<i32>::new();
    let mut freq = HashMap::<i32, i32>::new();

    for line in fs::read_to_string(INPUT).unwrap().lines() {
        let mut iter = line.split_ascii_whitespace();
        let x = iter.next();
        let x = x.unwrap().parse::<i32>().unwrap();
        xs.push(x);

        let y = iter.next();
        let y = y.unwrap().parse::<i32>().unwrap();
        freq.entry(y)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut res = 0;
    for x in xs.iter() {
        if let Some(f) = freq.get(x) {
            res += x * f
        }
    }

    println!("day1 part2 solution: {res}");
}
