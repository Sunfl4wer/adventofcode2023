use crate::utils;
use std::cmp::Ordering;
use std::collections;

pub fn part2() -> i64 {
    //let content = utils::read_file("./data/testday7.txt");
    let content = utils::read_file("./data/day7.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();
    let mut indexing: Vec<(i32, String, i64)> = lines
        .iter()
        .map(|s| s.split(" ").collect())
        .map(|s: Vec<&str>| {
            (
                hand_type_p2(&sortable_string_p2(s[0])),
                sortable_string_p2(s[0]),
                utils::to_num_i64(s[1].to_string()),
            )
        })
        .collect();

    indexing.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        other => other,
    });

    let mut res = 0;
    for i in 0..indexing.len() {
        let idx = &indexing[i];
        println!("{} {} {}", idx.0, idx.1, idx.2);
        res += idx.2 * ((i + 1) as i64);
    }

    res
}

fn sortable_string_p2(s: &str) -> String {
    s.chars()
        .map(|c| {
            let cv: char = match c {
                'J' => 'a',
                '2' => 'b',
                '3' => 'c',
                '4' => 'd',
                '5' => 'e',
                '6' => 'f',
                '7' => 'g',
                '8' => 'h',
                '9' => 'i',
                'T' => 'j',
                'Q' => 'k',
                'K' => 'l',
                _ => 'm',
            };
            cv
        })
        .into_iter()
        .collect()
}

fn hand_type_p2(s: &str) -> i32 {
    let five_of_a_kind = 5;
    let four_of_a_kind = 4;
    let full_house = 3;
    let three_of_a_kind = 2;
    let two_pair = 1;
    let one_pair = 0;
    let high_card = -1;

    let chars: Vec<char> = s.chars().collect();

    let mut char_count: collections::HashMap<char, usize> = collections::HashMap::new();
    for x in &chars {
        *char_count.entry(*x).or_default() += 1;
    }

    let max_key = char_count
        .iter()
        .filter(|e| *e.0 != 'a')
        .max_by_key(|entry| entry.1);

    match max_key {
        Some(k) => {
            let key = *k.0;
            let replaced_chars: Vec<char> = (&chars)
                .iter()
                .map(|c| {
                    if *c == 'a' {
                        return key;
                    }
                    *c
                })
                .collect();

            char_count = collections::HashMap::new();
            for x in &replaced_chars {
                *char_count.entry(*x).or_default() += 1;
            }

            let c = char_count.len();

            if c == 5 {
                return high_card;
            }
            if c == 4 {
                return one_pair;
            }
            if c == 1 {
                return five_of_a_kind;
            }
            if c == 3 {
                let no_twos = char_count.values().filter(|v| (2 as usize) == **v).count();
                if no_twos == 2 {
                    return two_pair;
                }
                return three_of_a_kind;
            }
            if c == 2 {
                let no_threes = char_count.values().filter(|v| (3 as usize) == **v).count();
                if no_threes == 1 {
                    return full_house;
                }
            }

            return four_of_a_kind;
        }
        _ => five_of_a_kind,
    }
}

pub fn part1() -> i64 {
    //let content = utils::read_file("./data/testday7.txt");
    let content = utils::read_file("./data/day7.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| *s != "").collect();
    let mut indexing: Vec<(i32, String, i64)> = lines
        .iter()
        .map(|s| s.split(" ").collect())
        .map(|s: Vec<&str>| {
            (
                hand_type(&sortable_string(s[0])),
                sortable_string(s[0]),
                utils::to_num_i64(s[1].to_string()),
            )
        })
        .collect();

    indexing.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        other => other,
    });

    let mut res = 0;
    for i in 0..indexing.len() {
        let idx = &indexing[i];
        println!("{} {} {}", idx.0, idx.1, idx.2);
        res += idx.2 * ((i + 1) as i64);
    }

    res
}

fn sortable_string(s: &str) -> String {
    s.chars()
        .map(|c| {
            let cv: char = match c {
                '2' => 'a',
                '3' => 'b',
                '4' => 'c',
                '5' => 'd',
                '6' => 'e',
                '7' => 'f',
                '8' => 'g',
                '9' => 'h',
                'T' => 'i',
                'J' => 'j',
                'Q' => 'k',
                'K' => 'l',
                _ => 'm',
            };
            cv
        })
        .into_iter()
        .collect()
}

fn hand_type(s: &str) -> i32 {
    let five_of_a_kind = 5;
    let four_of_a_kind = 4;
    let full_house = 3;
    let three_of_a_kind = 2;
    let two_pair = 1;
    let one_pair = 0;
    let high_card = -1;

    let chars: Vec<char> = s.chars().collect();

    let mut char_count: collections::HashMap<char, usize> = collections::HashMap::new();
    for x in chars {
        *char_count.entry(x).or_default() += 1;
    }
    let c = char_count.len();

    if c == 5 {
        return high_card;
    }
    if c == 4 {
        return one_pair;
    }
    if c == 1 {
        return five_of_a_kind;
    }
    if c == 3 {
        let no_twos = char_count.values().filter(|v| (2 as usize) == **v).count();
        if no_twos == 2 {
            return two_pair;
        }
        return three_of_a_kind;
    }
    if c == 2 {
        let no_threes = char_count.values().filter(|v| (3 as usize) == **v).count();
        if no_threes == 1 {
            return full_house;
        }
    }

    return four_of_a_kind;
}
