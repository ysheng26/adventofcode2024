use std::fs;

const INPUT: &str = "src/day2/input.txt";

// 1. levels needs to be all increasing or all decreasing
// 2. levels differ at least one and at most three
// return how many reports are safe

#[allow(dead_code)]
pub fn part1() {
    let mut res = 0;
    for line in fs::read_to_string(INPUT).unwrap().lines() {
        let iter = line.split_ascii_whitespace();
        let xs: Vec<i32> = iter.map(|x| x.parse::<i32>().unwrap()).collect();
        if is_safe(&xs) {
            res += 1;
        }
    }
    println!("day2 part1 solution: {res}");
}

#[allow(dead_code)]
pub fn part2() {
    let mut res = 0;
    for line in fs::read_to_string(INPUT).unwrap().lines() {
        let iter = line.split_ascii_whitespace();
        let xs: Vec<i32> = iter.map(|x| x.parse::<i32>().unwrap()).collect();

        if is_safe(&xs) {
            res += 1;
            continue;
        }

        // try removing elements till we find a safe one
        for i in 0..xs.len() {
            let mut xs = xs.clone();
            xs.remove(i);
            if is_safe(&xs) {
                res += 1;
                break;
            }
        }
    }
    println!("day2 part1 solution: {res}");
}

fn is_safe(xs: &[i32]) -> bool {
    // 0 1 2 3 4 5
    // 7 6 4 2 1
    //     i

    let mut i = 2;
    while i < xs.len() {
        let a = xs[i - 2];
        let b = xs[i - 1];
        let c = xs[i];

        if a <= b && b >= c {
            return false;
        } else if a >= b && b <= c {
            return false;
        }

        let diff1 = (a - b).abs();
        let diff2 = (b - c).abs();
        if diff1 > 3 || diff2 > 3 {
            return false;
        }

        i += 1;
    }

    true
}
