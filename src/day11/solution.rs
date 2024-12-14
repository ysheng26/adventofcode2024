use std::{collections::HashMap, fs};

use itertools::Itertools;

const INPUT: &str = "src/day11/input.txt";

// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.

fn has_even_digits(x: &i128) -> bool {
    x.to_string().len() % 2 == 0
}

fn split_num(x: &i128) -> Vec<i128> {
    let x = x.to_string();
    let (left, right) = x.split_at(x.len() / 2);
    let left = left.parse::<i128>().unwrap();
    let right = right.parse::<i128>().unwrap();

    vec![left, right]
}

fn blink(xs: Vec<i128>) -> Vec<i128> {
    let xs = xs
        .iter()
        .flat_map(|x| match x {
            0 => vec![1],
            x => {
                if has_even_digits(x) {
                    split_num(x)
                } else {
                    vec![x * 2024]
                }
            }
        })
        .collect_vec();
    xs
}

#[allow(dead_code)]
pub fn part1() {
    let mut input = fs::read_to_string(INPUT)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i128>().unwrap())
        .collect_vec();

    for _ in 0..25 {
        input = blink(input);
    }

    let res: usize = input.len();
    println!("day11 part1 solution: {res}");
}

fn blink_stone(stone: i128, num: i128, memo: &mut HashMap<(i128, i128), i128>) -> i128 {
    if num == 0 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(stone, num)) {
        return cached;
    }

    let mut res = 0;
    if stone == 0 {
        let curr = blink_stone(1, num - 1, memo);
        res += curr;
        memo.insert((1, num - 1), curr);
    } else if has_even_digits(&stone) {
        let tmp = split_num(&stone);
        let (a, b) = (tmp[0], tmp[1]);
        let curr = blink_stone(a, num - 1, memo);
        res += curr;
        memo.insert((a, num - 1), curr);

        let curr = blink_stone(b, num - 1, memo);
        res += curr;
        memo.insert((b, num - 1), curr);
    } else {
        let curr = blink_stone(stone * 2024, num - 1, memo);
        res += curr;
        memo.insert((stone * 2024, num - 1), curr);
    }
    res
}

#[allow(dead_code)]
pub fn part2() {
    let input = fs::read_to_string(INPUT)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i128>().unwrap())
        .collect_vec();

    let mut res = 0;
    let mut memo = HashMap::<(i128, i128), i128>::new();
    for stone in &input {
        res += blink_stone(*stone, 75, &mut memo);
    }

    println!("day11 part2 solution: {res}");
}
