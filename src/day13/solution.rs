use std::fs;

use itertools::Itertools;
use regex::Regex;

// const INPUT: &str = "src/day13/input.txt";
const INPUT: &str = "src/day13/input-small.txt";

#[derive(Debug, Default)]
struct Question {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    x: f64,
    y: f64,
}

fn is_whole(a: f64) -> bool {
    if (a - a.round()).abs() < 0.000000001 {
        return true;
    }
    false
}

fn is_zero(a: f64) -> bool {
    if a.abs() < 0.000000001 {
        return true;
    }
    false
}

impl Question {
    fn new(ax: f64, ay: f64, bx: f64, by: f64, x: f64, y: f64) -> Self {
        Question {
            ax,
            ay,
            bx,
            by,
            x,
            y,
        }
    }

    fn solve(&self) -> Option<f64> {
        let denom = self.bx * self.ay - self.by * self.ax;
        if is_zero(denom) {
            return None;
        }

        let res_a = (self.bx * self.y - self.by * self.x) / denom;
        let res_b = (self.x - self.ax * res_a) / self.bx;
        if res_a > 100f64 || res_b >= 100f64 {
            return None;
        }

        if !is_whole(res_a) || !is_whole(res_b) {
            return None;
        }

        let total = 3f64 * res_a + res_b;
        Some(total)
    }

    fn solve2(&self) -> Option<f64> {
        let denom = self.bx * self.ay - self.by * self.ax;
        if is_zero(denom) {
            return None;
        }

        let res_a = (self.bx * self.y - self.by * self.x) / denom;
        let res_b = (self.x - self.ax * res_a) / self.bx;

        if !is_whole(res_a) || !is_whole(res_b) {
            return None;
        }

        let total = 3f64 * res_a + res_b;
        Some(total)
    }
}

impl From<String> for Question {
    //             ax    ay             bx    by          x       y
    // Button A: X+94, Y+34|Button B: X+22, Y+67|Prize: X=8400, Y=5400
    fn from(value: String) -> Self {
        let mut q = Question::default();
        let re = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)\|Button B: X\+(\d+), Y\+(\d+)\|Prize: X\=(\d+), Y\=(\d+)$").unwrap();
        for (_, [ax, ay, bx, by, x, y]) in re.captures_iter(&value).map(|c| c.extract()) {
            q = Question {
                ax: ax.parse::<f64>().unwrap(),
                ay: ay.parse::<f64>().unwrap(),
                bx: bx.parse::<f64>().unwrap(),
                by: by.parse::<f64>().unwrap(),
                x: x.parse::<f64>().unwrap(),
                y: y.parse::<f64>().unwrap(),
            };
        }
        q
    }
}

#[allow(unused)]
pub fn part1() {
    let input = fs::read_to_string(INPUT).unwrap();
    let chunks = input.lines().chunks(4);

    let mut questions = Vec::<Question>::new();
    for mut chunk in chunks.into_iter() {
        let question = vec![
            chunk.next().unwrap(),
            chunk.next().unwrap(),
            chunk.next().unwrap(),
        ]
        .into_iter()
        .join("|");

        let question: Question = question.into();
        // println!("{:?}", question);
        questions.push(question);
    }

    let mut res = 0f64;
    for q in questions {
        if let Some(answer) = q.solve() {
            res += answer;
        }
    }
    res = res.round();

    println!("day13 part1 solution: {res}");
}

#[allow(unused)]
pub fn part2() {
    let input = fs::read_to_string(INPUT).unwrap();
    let chunks = input.lines().chunks(4);

    let mut questions = Vec::<Question>::new();
    for mut chunk in chunks.into_iter() {
        let question = vec![
            chunk.next().unwrap(),
            chunk.next().unwrap(),
            chunk.next().unwrap(),
        ]
        .into_iter()
        .join("|");

        let mut question: Question = question.into();
        question.x += 10000000000000f64;
        question.y += 10000000000000f64;
        questions.push(question);
    }

    let mut res = 0f64;
    for q in questions {
        println!("{:?}", q);
        if let Some(answer) = q.solve2() {
            res += answer;
        }
    }
    res = res.round();

    println!("day13 part2 solution: {res}");
}
