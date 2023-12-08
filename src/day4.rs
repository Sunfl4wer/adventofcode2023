use crate::utils;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1() -> i32 {
    let content = utils::read_file("./data/day4.txt");
    //let content = utils::read_file("./data/testday4.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut res = 0;
    for card in lines {
        if card == "" {
            continue;
        }
        let scores = winning_scores(card);
        res += scores;
    }
    res
}

fn winning_scores(line: &str) -> i32 {
    let mut res: i32 = 0;
    let card_nums: Vec<&str> = line.split(":").collect();
    if card_nums.len() == 0 {
        return 0;
    }
    let winning_guessing: Vec<&str> = card_nums[1].split("|").map(|s| s.trim()).collect();

    let winning: Vec<i32> = winning_guessing[0]
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| utils::to_num(s.to_string()))
        .collect();
    let mut guessing: Vec<i32> = winning_guessing[1]
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.trim())
        .map(|s| utils::to_num(s.to_string()))
        .collect();

    let m = guessing.len();

    let set: HashSet<_> = guessing.drain(..).collect(); // dedup
    guessing.extend(set.into_iter());

    let n = guessing.len();
    if m != n {
        println!("line {} has duplicate", line);
        utils::print_vec(&guessing);
    }

    let mut winning_map: HashMap<i32, bool> = HashMap::new();
    for n in winning {
        winning_map.insert(n, true);
    }

    let mut res = 0;
    for n in guessing {
        match winning_map.get(&n) {
            Some(true) => {
                if res == 0 {
                    res += 1;
                    continue;
                }
                res *= 2;
            }
            _ => {
                continue;
            }
        }
    }

    res
}

pub fn part2() -> i32 {
    let content = utils::read_file("./data/day4.txt");
    //let content = utils::read_file("./data/testday4.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();
    let mut res = 0;
    let mut copies: Vec<i32> = Vec::new();
    for _i in 0..lines.len() {
        copies.push(1);
    }
    for i in 0..lines.len() {
        let card = lines[i];
        let no_winning = count_cards(card);
        let c: i32 = copies[i];
        let end = cmp::min(lines.len(), i + 1 + no_winning as usize);
        for j in (i + 1)..end {
            copies[j] += c;
        }
    }

    copies.iter().sum()
}

fn count_cards(card: &str) -> i32 {
    let card_nums: Vec<&str> = card.split(":").collect();
    if card_nums.len() == 0 {
        return 0;
    }
    let winning_guessing: Vec<&str> = card_nums[1].split("|").map(|s| s.trim()).collect();

    let winning: Vec<i32> = winning_guessing[0]
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| utils::to_num(s.to_string()))
        .collect();
    let mut guessing: Vec<i32> = winning_guessing[1]
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.trim())
        .map(|s| utils::to_num(s.to_string()))
        .collect();

    let set: HashSet<_> = guessing.drain(..).collect(); // dedup
    guessing.extend(set.into_iter());

    let mut winning_map: HashMap<i32, bool> = HashMap::new();
    for n in winning {
        winning_map.insert(n, true);
    }

    let mut res = 0;
    for n in guessing {
        match winning_map.get(&n) {
            Some(true) => {
                res += 1;
            }
            _ => {
                continue;
            }
        }
    }

    res
}
