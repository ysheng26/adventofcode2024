use std::{collections::HashMap, fs};

const INPUT: &str = "src/day7/input.txt";

// 3749, 11387
// const INPUT: &str = "src/day7/input-small.txt";

/*

target should be i128

parse input first

*/

enum Op {
    Plus,
    Mul,
    Concat,
}

/*

0
5 2 12

*/

fn can_be_calcualted(target: i128, curr: i128, nums: &[i128], idx: usize, op: Op) -> bool {
    if curr == target {
        return true;
    }

    if idx == nums.len() {
        return false;
    }

    match op {
        Op::Plus => {
            if can_be_calcualted(target, curr + nums[idx], nums, idx + 1, Op::Plus) {
                return true;
            }
            if can_be_calcualted(target, curr + nums[idx], nums, idx + 1, Op::Mul) {
                return true;
            }
        }
        Op::Mul => {
            if can_be_calcualted(target, curr * nums[idx], nums, idx + 1, Op::Plus) {
                return true;
            }
            if can_be_calcualted(target, curr * nums[idx], nums, idx + 1, Op::Mul) {
                return true;
            }
        }
        Op::Concat => return false,
    }

    false
}

#[allow(dead_code)]
pub fn part1() {
    let mut input = HashMap::<i128, Vec<i128>>::new();
    for line in fs::read_to_string(INPUT).unwrap().lines() {
        let target: i128 = line.split(": ").nth(0).unwrap().parse::<i128>().unwrap();
        let nums = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        input.insert(target, nums);
    }

    let mut res: i128 = 0;
    for (target, nums) in input {
        if can_be_calcualted(target, 0, &nums, 0, Op::Plus) {
            res += target;
        }
    }
    println!("day7 part1 solution: {res}");
}

/*

  i
0 1 2 3
1 2 3

curr = 0
can_be_calcualted_2(target, 1, 1)
    can_be_calcualted_2(target, 1, 1)



1 || 2 + 3

12 + 3

*/
fn can_be_calcualted_2(target: i128, curr: i128, nums: &[i128], idx: usize, ops: &[Op]) -> bool {
    if curr > target {
        return false;
    }

    if curr == target && idx == nums.len() {
        return true;
    }

    if idx == nums.len() {
        return false;
    }

    for op in ops {
        match op {
            Op::Plus => {
                if can_be_calcualted_2(
                    target,
                    curr + nums[idx],
                    nums,
                    idx + 1,
                    &[Op::Plus, Op::Mul, Op::Concat],
                ) {
                    return true;
                }
            }
            Op::Mul => {
                if can_be_calcualted_2(
                    target,
                    curr * nums[idx],
                    nums,
                    idx + 1,
                    &[Op::Plus, Op::Mul, Op::Concat],
                ) {
                    return true;
                }
            }
            Op::Concat => {
                let next_curr = format!("{}{}", curr.to_string(), nums[idx])
                    .parse::<i128>()
                    .unwrap();
                if can_be_calcualted_2(
                    target,
                    next_curr,
                    nums,
                    idx + 1,
                    &[Op::Plus, Op::Mul, Op::Concat],
                ) {
                    return true;
                }
            }
        }
    }

    false
}

#[allow(dead_code)]
pub fn part2() {
    let mut input = HashMap::<i128, Vec<i128>>::new();
    for line in fs::read_to_string(INPUT).unwrap().lines() {
        let target: i128 = line.split(": ").nth(0).unwrap().parse::<i128>().unwrap();
        let nums = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        input.insert(target, nums);
    }

    let mut res: i128 = 0;
    for (target, nums) in input {
        if can_be_calcualted_2(target, 0, &nums, 0, &[Op::Plus]) {
            res += target;
        }
    }
    println!("day7 part2 solution: {res}");
}
