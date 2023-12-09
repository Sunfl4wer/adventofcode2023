use crate::utils;

pub fn part1() -> usize {
    //let content = utils::read_file("./data/testday6.txt");
    let content = utils::read_file("./data/day6.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let times: Vec<i32> = (&lines[0])
        .split(":")
        .map(|s| s.trim())
        .filter(|s| !s.starts_with("Time"))
        .map(|s| s.split(" "))
        .flatten()
        .filter(|s| *s != "")
        .map(|s| utils::to_num(s.to_string()))
        .collect();

    let distances: Vec<i32> = (&lines[1])
        .split(":")
        .map(|s| s.trim())
        .filter(|s| !s.starts_with("Distance"))
        .map(|s| s.split(" "))
        .flatten()
        .filter(|s| *s != "")
        .map(|s| utils::to_num(s.to_string()))
        .collect();

    let mut res = 1;
    for i in 0..times.len() {
        let time = times[i];
        let record_distance = distances[i];
        let possibilities: usize = (0..time + 1)
            .map(|pressing_time| {
                let traveling_time = time - pressing_time;
                pressing_time * traveling_time
            })
            .filter(|travelling_distance| travelling_distance > &record_distance)
            .count();
        println!("Possibilities: {}", possibilities);
        res *= possibilities;
    }

    res
}

pub fn part2() -> usize {
    //let content = utils::read_file("./data/testday6.txt");
    let content = utils::read_file("./data/day6.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();

    let times: Vec<i64> = (&lines[0])
        .split(":")
        .map(|s| s.trim())
        .filter(|s| !s.starts_with("Time"))
        .map(|s| {
            let mut v = s.to_string();
            v.retain(|c| !c.is_whitespace());
            println!("v: {}", v);
            v
        })
        .map(|s| utils::to_num_i64(s))
        .collect();

    let distances: Vec<i64> = (&lines[1])
        .split(":")
        .map(|s| s.trim())
        .filter(|s| !s.starts_with("Distance"))
        .map(|s| {
            let mut v = s.to_string();
            v.retain(|c| !c.is_whitespace());
            v
        })
        .map(|s| utils::to_num_i64(s))
        .collect();

    let mut res = 1;
    for i in 0..times.len() {
        let time = times[i];
        let record_distance = distances[i];
        let possibilities: usize = (0..time + 1)
            .map(|pressing_time| {
                let traveling_time = time - pressing_time;
                pressing_time * traveling_time
            })
            .filter(|travelling_distance| travelling_distance > &record_distance)
            .count();
        println!("Possibilities: {}", possibilities);
        res *= possibilities;
    }

    res
}
