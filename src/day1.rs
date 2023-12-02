use crate::utils;

pub fn part1() -> i32 {
    let corpus: Vec<String> = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .iter()
        .map(|&s| s.to_string())
        .collect();
    day1(&corpus)
}

pub fn part2() -> i32 {
    let corpus: Vec<String> = Vec::from([
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ])
    .iter()
    .map(|&s| s.to_string())
    .collect();
    day1(&corpus)
}

fn day1(corpus: &[String]) -> i32 {
    let content = utils::read_file("./data/day1.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut res: i32 = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let mut s = line;
        let mut r = line;
        let mut sv = 0;
        let mut rv = 0;
        while s != "" {
            let n = starts_with_num(s, &corpus);
            if n != -1 {
                sv = n;
                break;
            }
            s = &s[1..];
        }
        while r != "" {
            let n = ends_with_num(r, &corpus);
            if n != -1 {
                rv = n;
                break;
            }
            r = &r[..r.len() - 1];
        }
        let v = sv * 10 + rv;
        res += v;
    }
    res
}

fn ends_with_num(s: &str, corpus: &[String]) -> i32 {
    let mut res: i32 = -1;
    for c in corpus {
        if s.ends_with(c) {
            match c.as_ref() {
                "one" | "1" => {
                    res = 1;
                }
                "two" | "2" => {
                    res = 2;
                }
                "three" | "3" => {
                    res = 3;
                }
                "four" | "4" => {
                    res = 4;
                }
                "five" | "5" => {
                    res = 5;
                }
                "six" | "6" => {
                    res = 6;
                }
                "seven" | "7" => {
                    res = 7;
                }
                "eight" | "8" => {
                    res = 8;
                }
                "nine" | "9" => {
                    res = 9;
                }
                _ => {
                    continue;
                }
            }
        }
    }
    res
}

fn starts_with_num(s: &str, corpus: &[String]) -> i32 {
    let mut res: i32 = -1;
    for c in corpus {
        if s.starts_with(c) {
            match c.as_ref() {
                "one" | "1" => {
                    res = 1;
                }
                "two" | "2" => {
                    res = 2;
                }
                "three" | "3" => {
                    res = 3;
                }
                "four" | "4" => {
                    res = 4;
                }
                "five" | "5" => {
                    res = 5;
                }
                "six" | "6" => {
                    res = 6;
                }
                "seven" | "7" => {
                    res = 7;
                }
                "eight" | "8" => {
                    res = 8;
                }
                "nine" | "9" => {
                    res = 9;
                }
                _ => {
                    continue;
                }
            }
        }
    }
    res
}
