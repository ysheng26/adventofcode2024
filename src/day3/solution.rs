use std::fs;

use regex::Regex;

const INPUT: &str = "src/day3/input.txt";

#[allow(dead_code)]
pub fn part1() {
    let input = fs::read_to_string(INPUT).unwrap();
    let res = calc_mul(&input);
    println!("day3 part1 solution: {res}");
}

fn calc_mul(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        res += a * b;
    }
    res
}

#[allow(dead_code)]
pub fn part2() {
    /*
    split by do() first

    then for each split by don't()

    take the first element and calc_mul
    */

    let input = fs::read_to_string(INPUT).unwrap();
    let re = Regex::new(r"do\(\)").unwrap();
    let split_by_do: Vec<&str> = re.split(&input).collect();

    let mut res = 0;
    for xs in split_by_do {
        let re = Regex::new(r"don't\(\)").unwrap();
        let split_by_dont: Vec<&str> = re.split(xs).collect();
        if let Some(&x) = split_by_dont.first() {
            res += calc_mul(x)
        }
    }

    println!("day3 part1 solution: {res}");
}
