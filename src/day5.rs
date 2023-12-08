use crate::utils;
use std::collections;

pub fn part2() -> i64 {
    //let content = utils::read_file("./data/day5.txt");
    let content = utils::read_file("./data/testday5.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let mut nums: Vec<i64> = Vec::new();
    let mut value_mapping: [Vec<[i64; 3]>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut col: i8 = -1;
    for i in 0..lines.len() {
        let line = lines[i];
        if line.starts_with("seeds:") {
            let label_nums: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
            nums = label_nums[1]
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| *s != "")
                .map(|s| utils::to_num_i64(s.to_string()))
                .collect();
            continue;
        }
        if line.contains("map:") {
            col += 1;
            continue;
        }
        let nums: Vec<i64> = line
            .split(" ")
            .map(|s| utils::to_num_i64(s.to_string()))
            .collect();
        let v: [i64; 3] = [nums[0], nums[1], nums[2]];
        value_mapping[col as usize].push(v);
    }

    use std::thread;
    let mut res: Vec<std::thread::JoinHandle<i64>> = Vec::new();
    for i in (0..nums.len()).step_by(2) {
        let value_mapping = value_mapping.clone();
        let n = nums[i];
        let count = nums[i + 1];
        res.push(thread::spawn(move || {
            let i = i;
            println!("[{}] start handling seeds: {} - {}", i / 2, n, count);
            let mut min_loc = i64::MAX;
            for d in 0..count {
                let mut v = n + d;
                for k in 0..value_mapping.len() {
                    let mut new_v = v;
                    let mapping = &value_mapping[k];
                    for j in 0..mapping.len() {
                        let m: [i64; 3] = mapping[j];
                        let dest = m[0];
                        let src = m[1];
                        let c = m[2];
                        let t = v - src;
                        if t >= 0 && t < c {
                            new_v = dest + t;
                            break;
                        }
                    }
                    v = new_v;
                }

                if min_loc > v {
                    min_loc = v;
                }
            }
            println!("[{}] finish handling seeds: {} - {}", i / 2, n, count);
            min_loc
        }));
    }

    let mut min_loc = i64::MAX;
    for jh in res {
        let t = jh.join().unwrap();
        if min_loc > t {
            min_loc = t;
        }
    }
    min_loc
}

pub fn part1() -> i64 {
    let content = utils::read_file("./data/day5.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();
    let location: usize = 7;

    let mut planting_map: Vec<[i64; 8]> = Vec::new();

    let mut col = 0;
    let mut nums: Vec<i64> = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        if line.starts_with("seeds:") {
            let label_nums: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
            nums = label_nums[1]
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| *s != "")
                .map(|s| utils::to_num_i64(s.to_string()))
                .collect();
            continue;
        }
        if line.contains("map:") {
            println!("{}", line);
            for j in 0..planting_map.len() {
                let mut p = planting_map[j];
                if p[col] == -1 {
                    p[col] = p[col - 1];
                }
                planting_map[j] = p;
            }
            col += 1;
            continue;
        }
        let nums: Vec<i64> = line
            .split(" ")
            .map(|s| utils::to_num_i64(s.to_string()))
            .collect();
        for j in 0..planting_map.len() {
            let mut p = planting_map[j];
            let src_val = p[col - 1 as usize];
            let src_begin = nums[1];
            let d = src_val - src_begin;
            println!("line: {}", line);
            println!("src: {} - d: {}", src_val, d);
            if d < 0 || d >= nums[2] {
                continue;
            }
            println!(
                "last val: {} - last begin: {} - d: {} - num[2]: {}",
                src_val, src_begin, d, nums[2],
            );

            let dest_val = nums[0] + d;
            p[col] = dest_val;
            planting_map[j] = p;
        }
    }

    for j in 0..planting_map.len() {
        let mut p = planting_map[j];
        if p[col] == -1 {
            p[col] = p[col - 1];
        }
        planting_map[j] = p;
    }
    planting_map.iter().map(|s| s[location]).min().unwrap()
}
