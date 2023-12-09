use crate::utils;
use std::collections;

pub fn part1() -> i32 {
    //let content = utils::read_file("./data/testday8.txt");
    let content = utils::read_file("./data/day8.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let mut network: collections::HashMap<String, (String, String)> = collections::HashMap::new();

    let mut instructions: Vec<char> = Vec::new();

    let mut first_line = true;
    for line in lines {
        if first_line {
            instructions = line.chars().collect();
            first_line = false;
            continue;
        }
        let direction: Vec<String> = line
            .split("=")
            .map(|s| s.trim())
            .map(|s| s.replace("(", ""))
            .map(|s| s.replace(")", ""))
            .map(|s| {
                let v: Vec<String> = s.split(",").map(|s| s.to_owned()).collect();
                v
            })
            .flatten()
            .filter(|s| *s != "")
            .map(|s: String| {
                let trimed_s = (&s).trim().to_owned();
                trimed_s
            })
            .collect();

        if line.contains("AAA") {
            println!(
                "direction {}: {} {}",
                direction[0], direction[1], direction[2]
            );
        }

        network.insert(
            direction[0].to_owned(),
            (direction[1].to_owned(), direction[2].to_owned()),
        );
    }

    println!("network len: {}", network.len());
    for k in network.keys() {
        match network.get(k) {
            Some(v) => {
                println!("{}: {} {}", k, v.0, v.1);
            }
            _ => {}
        }
    }

    let mut current = "AAA".to_string();
    let target = "ZZZ".to_string();
    let mut res: i32 = 0;
    while current != target {
        println!("current: {}", current);
        let left_right = network.get(&current).unwrap();
        match instructions[(res % instructions.len() as i32) as usize] {
            'L' => current = left_right.0.to_owned(),
            'R' => current = left_right.1.to_owned(),
            _ => {}
        }
        res += 1;
    }

    res
}

pub fn part2() -> i64 {
    //let content = utils::read_file("./data/testday8.txt");
    let content = utils::read_file("./data/day8.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let mut network: collections::HashMap<String, (String, String)> = collections::HashMap::new();

    let mut instructions: Vec<char> = Vec::new();

    let mut first_line = true;
    for line in lines {
        if first_line {
            instructions = line.chars().collect();
            first_line = false;
            continue;
        }
        let direction: Vec<String> = line
            .split("=")
            .map(|s| s.trim())
            .map(|s| s.replace("(", ""))
            .map(|s| s.replace(")", ""))
            .map(|s| {
                let v: Vec<String> = s.split(",").map(|s| s.to_owned()).collect();
                v
            })
            .flatten()
            .filter(|s| *s != "")
            .map(|s: String| {
                let trimed_s = (&s).trim().to_owned();
                trimed_s
            })
            .collect();

        network.insert(
            direction[0].to_owned(),
            (direction[1].to_owned(), direction[2].to_owned()),
        );
    }

    let mut currents: Vec<String> = network
        .keys()
        .filter(|s| s.ends_with("A"))
        .map(|s| s.to_owned())
        .collect();

    let mut steps: Vec<i64> = Vec::new();

    for i in 0..currents.len() {
        let mut current = currents[i].to_owned();
        let mut res: i64 = 0;
        while !current.ends_with("Z") {
            let left_right = network.get(&current).unwrap();
            match instructions[(res % instructions.len() as i64) as usize] {
                'L' => current = left_right.0.to_owned(),
                'R' => current = left_right.1.to_owned(),
                _ => {}
            }
            res += 1;
        }
        steps.push(res);
    }

    utils::lcmm(&steps)
}

fn all_reach_destinations(v1: &[String]) -> bool {
    for v in v1 {
        if !v.ends_with("Z") {
            return false;
        }
    }
    return true;
}
