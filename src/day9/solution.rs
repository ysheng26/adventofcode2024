use std::fs;

const INPUT: &str = "src/day9/input.txt";
// const INPUT: &str = "src/day9/input-small.txt";
// const INPUT: &str = "src/day9/input-small2.txt";

fn two_pointer(xs: Vec<String>) -> Vec<String> {
    let mut xs = xs.clone();

    let (mut i, mut j) = (0, xs.len() - 1);

    while i < j {
        while i < xs.len() && xs[i] != "." {
            i += 1;
        }

        while xs[j] == "." {
            j -= 1;
        }
        xs.swap(i, j);
        i += 1;
        j -= 1;
    }
    xs
}

fn checksum(xs: &[String]) -> usize {
    xs.into_iter()
        .enumerate()
        .map(|x| x.0 * x.1.parse::<usize>().unwrap_or(0))
        .fold(0, |acc, x| acc + x)
}

#[allow(dead_code)]
pub fn part1() {
    let input = fs::read_to_string(INPUT).unwrap();

    let mut res = Vec::<String>::new();
    /*
    char,freespace,id

    0 1 2 3 4 5
    1 2 3 4 5

    */
    let n = input.chars().count();
    let mut i = 0;
    while i < n {
        // block_size, free_space, filename is i
        let block_size = input.chars().nth(i);
        let free_space = input.chars().nth(i + 1);

        if block_size.is_some() {
            // println!("i is {i}");
            let filename = (i / 2).to_string();
            let block_size = block_size.unwrap().to_digit(10).unwrap();
            for _ in 0..block_size {
                res.push(filename.clone());
            }
        }

        if free_space.is_some() {
            let free_space = free_space.unwrap().to_digit(10).unwrap();
            for _ in 0..free_space {
                res.push(".".to_string());
            }
        }
        i += 2
    }

    let res = two_pointer(res);
    // println!("part1 res {:?}", res);
    let res = checksum(&res);
    println!("day9 part1 solution: {res}");
}

#[derive(Debug, Clone, PartialEq)]
struct Blob {
    size: usize,
    is_empty: bool,
    filename: Option<String>,
}

impl Blob {
    fn new(size: usize, is_empty: bool, filename: Option<String>) -> Self {
        Blob {
            size,
            is_empty,
            filename,
        }
    }

    // optional blob is the extra free space
    fn put_file_in_slot(&mut self, file: Blob) -> Option<Blob> {
        self.is_empty = false;
        self.filename = file.filename;

        let orig_size = self.size;

        if self.size > file.size {
            self.size = file.size;
            return Some(Blob {
                is_empty: true,
                filename: None,
                size: orig_size - file.size,
            });
        }
        None
    }
}

#[allow(dead_code)]
pub fn part2() {
    let input = fs::read_to_string(INPUT).unwrap();
    let mut res = Vec::<Blob>::new();
    /*
    char,freespace,id
    */
    let n = input.chars().count();
    let mut i = 0;
    while i < n {
        // block_size, free_space, filename is i
        let block_size = input.chars().nth(i);
        let free_space = input.chars().nth(i + 1);

        if block_size.is_some() {
            // println!("i is {i}");
            let filename = (i / 2).to_string();
            let block_size = block_size.unwrap().to_digit(10).unwrap() as usize;
            res.push(Blob::new(block_size, false, Some(filename)));
        }

        if free_space.is_some() {
            let free_space = free_space.unwrap().to_digit(10).unwrap() as usize;
            res.push(Blob::new(free_space, true, None));
        }
        i += 2
    }

    // println!("{:?}", res);

    /*
    from right to left, for each file ID
        try to find a place from left to right that is >= fileID size
            if not found skip
            if found
                clear fileID and split existing Blob
    */

    /*
    00...111...2...333.44.5555.6666.777.888899
    */

    let files: Vec<Blob> = res
        .clone()
        .into_iter()
        .filter(|blob| !blob.is_empty)
        .rev()
        .collect();

    for f in files {
        let j = res
            .iter()
            .position(|blob| !blob.is_empty && *blob == f)
            .unwrap();

        if let Some(i) = res
            .iter()
            .position(|blob| blob.is_empty && blob.size >= f.size)
        {
            // if new memeory position is to the right of current file
            // break
            if i >= j {
                continue;
            }

            // we found the slot res[i], put res[i] in and maybe insert
            let ff = res[j].clone();
            res[j].filename = None;
            res[j].is_empty = true;

            if let Some(extra_space) = res[i].put_file_in_slot(ff) {
                res.insert(i + 1, extra_space);
            }
        }
    }
    // println!("{:?}", res);

    let res = res
        .iter()
        .flat_map(|blob| {
            if blob.is_empty {
                return "."
                    .repeat(blob.size)
                    .chars()
                    .map(|ch| ch.to_string())
                    .collect();
            } else {
                // println!("blob.filename {:?}", blob.filename);
                let filename = blob.filename.clone().unwrap();
                let mut tmp = vec![];
                for _ in 0..blob.size {
                    tmp.push(filename.clone());
                }
                tmp
            }
        })
        .collect::<Vec<String>>();

    // println!("{:?}", res);

    // how to with iterators?

    // println!("{:?}", res);

    let res = checksum(&res);

    // let res = res.join("");
    // println!("should be: 00992111777.44.333....5555.6666.....8888..");
    // println!("    I got: {res}");

    // let x: Vec<String> = res.chars().map(|x| x.to_string()).collect();
    // 2858
    println!("day9 part2 solution: {res}");
}
