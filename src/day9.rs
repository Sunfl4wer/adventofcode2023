use crate::utils;
use std::collections;

pub fn part1() -> i64 {
    //let content = utils::read_file("./data/testday9.txt");
    let content = utils::read_file("./data/day9.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let histories: Vec<Vec<i64>> = lines
        .iter()
        .map(|s| {
            let v: Vec<i64> = s
                .to_string()
                .split(" ")
                .filter(|s| *s != "")
                .map(|c| c.trim())
                .map(|s| utils::to_num_i64(s.to_string()))
                .collect();
            v
        })
        .collect();
    let mut res: i64 = 0;
    let mut diff: Vec<i64> = Vec::new();

    for h in histories {
        let mut tracking: Vec<Vec<i64>> = Vec::new();
        tracking.push(h);
        loop {
            let v = &tracking[tracking.len() - 1];
            if v.len() == 1 && !all_zeros(v) {
                diff.clear();
                break;
            }
            for i in 1..v.len() {
                diff.push(v[i] - v[i - 1]);
            }
            if all_zeros(&diff) {
                diff.clear();
                break;
            }
            tracking.push(diff.clone());
            diff.clear();
        }
        let mut res_v = 0;
        let mut j = (tracking.len() - 1) as i32;
        println!("tracking len {}", tracking.len());
        for t in &tracking {
            for v in t {
                print!("{} ", v);
            }
            println!("");
        }
        while j >= 0 {
            let cur = &tracking[j as usize];
            println!("[{}] cur len {}", j, cur.len());
            let last = cur[cur.len() - 1];
            res_v = last + res_v;
            j -= 1;
        }
        res += res_v;
    }

    res
}

pub fn part2() -> i64 {
    //let content = utils::read_file("./data/testday9.txt");
    let content = utils::read_file("./data/day9.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let histories: Vec<Vec<i64>> = lines
        .iter()
        .map(|s| {
            let v: Vec<i64> = s
                .to_string()
                .split(" ")
                .filter(|s| *s != "")
                .map(|c| c.trim())
                .map(|s| utils::to_num_i64(s.to_string()))
                .collect();
            v
        })
        .collect();
    let mut res: i64 = 0;
    let mut diff: Vec<i64> = Vec::new();

    for h in histories {
        let mut tracking: Vec<Vec<i64>> = Vec::new();
        tracking.push(h);
        loop {
            let v = &tracking[tracking.len() - 1];
            if v.len() == 1 && !all_zeros(v) {
                diff.clear();
                break;
            }
            for i in 1..v.len() {
                diff.push(v[i] - v[i - 1]);
            }
            if all_zeros(&diff) {
                diff.clear();
                break;
            }
            tracking.push(diff.clone());
            diff.clear();
        }
        let mut res_v = 0;
        let mut j = (tracking.len() - 1) as i32;
        println!("tracking len {}", tracking.len());
        for t in &tracking {
            for v in t {
                print!("{} ", v);
            }
            println!("");
        }
        while j >= 0 {
            let cur = &tracking[j as usize];
            println!("[{}] cur len {}", j, cur.len());
            let last = cur[0];
            res_v = last - res_v;
            j -= 1;
        }
        res += res_v;
    }

    res
}

fn all_zeros(v: &[i64]) -> bool {
    if v.len() == 0 {
        return true;
    }
    for n in v {
        if *n != 0 {
            return false;
        }
    }
    return true;
}

fn all_reach_destinations(v1: &[String]) -> bool {
    for v in v1 {
        if !v.ends_with("Z") {
            return false;
        }
    }
    return true;
}
