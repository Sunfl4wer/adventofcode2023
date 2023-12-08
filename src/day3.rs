use crate::utils;

pub fn part2() -> i32 {
    let content = utils::read_file("./data/day3.txt");
    //let content = utils::read_file("./data/testday3.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut res: i32 = 0;

    for i in 0..height {
        let mut l = lines[i];
        l = l.trim();
        let line: Vec<char> = l.chars().collect();
        for j in 0..line.len() {
            let c = line[j];
            match c {
                '*' => {
                    let mut nums: Vec<i32> = Vec::new();
                    nums.append(&mut find_nums(j, &line));
                    if i > 0 {
                        let above_line: Vec<char> = lines[i - 1].chars().collect();
                        nums.append(&mut find_nums(j, &above_line));
                    }
                    if i < height - 1 {
                        let below_line: Vec<char> = lines[i + 1].chars().collect();
                        nums.append(&mut find_nums(j, &below_line));
                    }
                    if nums.len() != 2 {
                        continue;
                    }
                    res += nums[0] * nums[1];
                }
                _ => {
                    continue;
                }
            }
        }
    }
    res
}

fn find_nums(x: usize, line: &[char]) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    let c = line[x];
    if c.is_digit(10) {
        let mut digits: Vec<char> = Vec::new();
        let mut start = x;
        while start > 0 && line[start - 1].is_digit(10) {
            start -= 1;
        }
        while start < line.len() && line[start].is_digit(10) {
            digits.push(line[start]);
            start += 1;
        }
        return vec![utils::chars_to_num(&digits)];
    }

    let mut lstart = (x as i32) - 1;
    let rstart = (x as i32) + 1;
    if rstart < line.len() as i32 && line[rstart as usize].is_digit(10) {
        let mut digits: Vec<char> = Vec::new();
        let mut j = rstart as usize;
        while j < line.len() && line[j].is_digit(10) {
            digits.push(line[j]);
            j += 1;
        }
        res.push(utils::chars_to_num(&digits));
    }

    if lstart >= 0 as i32 && line[lstart as usize].is_digit(10) {
        let mut digits: Vec<char> = Vec::new();
        while lstart > 0 && line[lstart as usize - 1].is_digit(10) {
            lstart -= 1;
        }
        while lstart < line.len() as i32 && line[lstart as usize].is_digit(10) {
            digits.push(line[lstart as usize]);
            lstart += 1;
        }
        res.push(utils::chars_to_num(&digits));
    }

    res
}

pub fn part1() -> i32 {
    let content = utils::read_file("./data/day3.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut symbols: Vec<Vec<i32>> = vec![vec![0; width]; height];
    for i in 0..height {
        let line: Vec<char> = lines[i].chars().collect();
        for j in 0..line.len() {
            let c: char = line[j];
            if c.is_digit(10) {
                continue;
            }
            match c {
                '.' | '\n' | '\r' => {
                    continue;
                }
                _ => {
                    symbols[i][j] = 1;
                }
            }
        }
    }

    let mut res: i32 = 0;

    for i in 0..height {
        let mut l = lines[i];
        l = l.trim();
        let line: Vec<char> = l.chars().collect();
        let mut digits: Vec<char> = Vec::new();
        if i >= 1 {
            println!("line[{}]: {}", i - 1, lines[i - 1]);
        }
        println!("line[{}]: {}", i, lines[i]);
        if i < height - 1 {
            println!("line[{}]: {}", i + 1, lines[i + 1]);
        }
        let mut line_sum: i32 = 0;
        for j in 0..line.len() {
            let c = line[j];
            if utils::is_digit(c) {
                digits.push(c);
            }
            if (!utils::is_digit(c) || j == line.len() - 1) && digits.len() != 0 {
                let mut end_idx = j;
                if !utils::is_digit(c) {
                    end_idx -= 1;
                }
                let start_idx = end_idx + 1 - digits.len();
                let adjacent_cells = adjacent_cells(start_idx as i32, end_idx as i32, i as i32);
                for cell in adjacent_cells {
                    let x = cell[0];
                    let y = cell[1];
                    if adjacent_a_symbol(x as i32, y as i32, &symbols[0..]) {
                        let v = utils::chars_to_num(&digits);
                        line_sum += v;
                        println!(
                            "num: {} line[{}] start: {} end: {} - symbol: line[{}] idx[{}]",
                            v, i, start_idx, end_idx, x, y
                        );
                        break;
                    }
                }
                digits = Vec::new();
            }
        }
        println!(
            "res: {} - line sum: {} - res: {}",
            res,
            line_sum,
            res + line_sum
        );
        res += line_sum;
    }
    res
}

fn adjacent_a_symbol(i: i32, j: i32, symbols: &[Vec<i32>]) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    let iusize = i as usize;
    let jusize = j as usize;
    if iusize >= symbols.len() || jusize >= symbols[0].len() {
        return false;
    }
    return symbols[iusize][jusize] == 1;
}

fn adjacent_cells(start: i32, end: i32, line: i32) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = Vec::new();
    let above_line = line - 1;
    let below_line = line + 1;
    for i in start..(end + 1) {
        if above_line >= 0 {
            let p1 = vec![above_line as usize, i as usize];
            res.push(p1);
        }
        if below_line >= 0 {
            let p3 = vec![below_line as usize, i as usize];
            res.push(p3);
        }
        let p2 = vec![line as usize, i as usize];
        res.push(p2);
    }
    if above_line >= 0 {
        if start >= 1 {
            res.push(vec![above_line as usize, (start - 1) as usize]);
        }
        res.push(vec![above_line as usize, (end + 1) as usize]);
    }

    if start >= 1 {
        res.push(vec![line as usize, (start - 1) as usize]);
        res.push(vec![below_line as usize, (start - 1) as usize]);
    }

    res.push(vec![line as usize, (end + 1) as usize]);
    res.push(vec![below_line as usize, (end + 1) as usize]);

    res
}
