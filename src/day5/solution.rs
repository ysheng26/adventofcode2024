use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const INPUT: &str = "src/day5/input.txt";
// const INPUT: &str = "src/day5/input-small.txt";

#[derive(Debug, Clone)]
struct Graph {
    adj_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::<i32, Vec<i32>>::new(),
        }
    }

    pub fn filter_nodes(&mut self, xs: &[i32]) {
        let all_nodes: HashSet<i32> = self.adj_list.keys().map(|x| *x).collect();
        let keep_nodes: HashSet<i32> = xs.iter().map(|x| *x).collect();
        let remove_nodes: HashSet<i32> = &all_nodes - &keep_nodes;

        for node in remove_nodes {
            self.adj_list.remove(&node);
        }

        for (_, vs) in &mut self.adj_list {
            vs.retain(|x| xs.contains(x));
        }
    }

    pub fn add_edge(&mut self, src: i32, dst: i32) {
        self.adj_list
            .entry(src)
            .and_modify(|xs| xs.push(dst))
            .or_insert(vec![dst]);
    }

    fn top_sort(&self) -> Vec<i32> {
        let mut indegree = HashMap::<i32, i32>::new();
        for (src, dst_list) in &self.adj_list {
            // all nodes start with indgree 0
            indegree.entry(*src).or_insert(0);
            for dst in dst_list {
                indegree.entry(*dst).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        // println!("{:?}", indegree);

        // get all indegree 0 nodes and spin up the queue
        let mut queue: VecDeque<i32> = indegree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(k, _)| *k)
            .collect();

        let mut res = Vec::<i32>::new();
        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            res.push(curr);
            // for east dst that curr has, -1 of the indegree
            // *indegree.get_mut(&curr).unwrap() -= 1;
            if let Some(dst_list) = self.adj_list.get(&curr) {
                for dst in dst_list {
                    *indegree.get_mut(dst).unwrap() -= 1;
                    // if the indegree after -1 is 0, add it to queue
                    if *indegree.get(dst).unwrap() == 0 {
                        queue.push_back(*dst);
                    }
                }
            }

            // println!("{:?}", indegree);
        }

        res
    }
}

#[allow(dead_code)]
pub fn part1() {
    // 1 read in graph
    // 2 do top sort on graph

    let content = fs::read_to_string(INPUT).unwrap();

    let mut all_instructions = Vec::<Vec<i32>>::new();
    let mut is_graph_input = true;
    let mut g = Graph::new();
    for line in content.lines() {
        if line == "" {
            is_graph_input = false;
            continue;
        }

        if is_graph_input {
            let src_dst: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            let src = src_dst[0];
            let dst = src_dst[1];
            g.add_edge(src, dst);
        } else {
            let instructions: Vec<i32> =
                line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            all_instructions.push(instructions);
        }
    }

    let mut res = 0;
    for instruction_list in all_instructions {
        if is_valid_instruction(&g, &instruction_list) {
            res += instruction_list[instruction_list.len() / 2]
        }
    }
    println!("day5 part1 solution: {res}");
}

fn is_valid_instruction(g: &Graph, instruction_list: &[i32]) -> bool {
    let mut g = g.clone();

    g.filter_nodes(&instruction_list);
    // println!("after {:?}", g);
    let topsort = g.top_sort();
    // println!("{:?}", topsort);

    let mut pos_list: Vec<usize> = vec![];
    for instruction in instruction_list {
        pos_list.push(topsort.iter().position(|curr| curr == instruction).unwrap());
    }
    let mut i = 0;
    while i < pos_list.len() - 1 {
        if pos_list[i] >= pos_list[i + 1] {
            return false;
        }
        i += 1;
    }
    true
}

fn fix_instruction(g: &Graph, mut instruction_list: Vec<i32>) -> Vec<i32> {
    let mut g = g.clone();

    g.filter_nodes(&instruction_list);
    // println!("after {:?}", g);
    let topsort = g.top_sort();
    // println!("{:?}", topsort);

    let mut pos_list: Vec<usize> = vec![];
    for instruction in instruction_list.iter() {
        pos_list.push(topsort.iter().position(|curr| curr == instruction).unwrap());
    }
    // topsort [a, b, c, d, e, f]
    // instruction_list [c, b, d, f]

    // sort instruction_list based on topsort ranking
    instruction_list.sort_by(|a, b| {
        let a_idx = topsort.iter().position(|x| x == a).unwrap();
        let b_idx = topsort.iter().position(|x| x == b).unwrap();
        a_idx.cmp(&b_idx)
    });

    instruction_list
}

#[allow(dead_code)]
pub fn part2() {
    let content = fs::read_to_string(INPUT).unwrap();

    let mut all_instructions = Vec::<Vec<i32>>::new();
    let mut is_graph_input = true;
    let mut g = Graph::new();
    for line in content.lines() {
        if line == "" {
            is_graph_input = false;
            continue;
        }

        if is_graph_input {
            let src_dst: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            let src = src_dst[0];
            let dst = src_dst[1];
            g.add_edge(src, dst);
        } else {
            let instructions: Vec<i32> =
                line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            all_instructions.push(instructions);
        }
    }

    let all_instructions: Vec<Vec<i32>> = all_instructions
        .iter()
        .filter(|xs| !is_valid_instruction(&g, xs))
        .cloned()
        .collect();

    let mut res = 0;
    for instruction_list in all_instructions {
        let xs = fix_instruction(&g, instruction_list);
        res += xs[xs.len() / 2]
    }
    println!("day5 part2 solution: {res}");
}
