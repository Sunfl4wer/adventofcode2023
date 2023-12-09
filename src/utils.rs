use std::fs;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("should has string")
}

pub fn print_vec(vtr: &[i32]) {
    for v in vtr {
        print!("{} ", v);
    }
    println!("");
}

pub fn to_num_i64(s: String) -> i64 {
    let cs: Vec<char> = s.chars().filter(|s| *s != '-').collect();
    let mut res: i64 = 0;
    let zero: i64 = '0' as i64;
    for c in cs {
        let v = c as i64 - zero;
        res = res * 10 + v;
    }
    let mut t = 1;
    if s.starts_with("-") {
        t = -1;
    }
    res * t
}

pub fn to_num(s: String) -> i32 {
    let cs: Vec<char> = s.chars().collect();
    let mut res: i32 = 0;
    let zero: i32 = '0' as i32;
    for c in cs {
        let v = c as i32 - zero;
        res = res * 10 + v;
    }
    res
}

pub fn chars_to_num(cs: &[char]) -> i32 {
    let mut res: i32 = 0;
    let zero: i32 = '0' as i32;
    for c in cs {
        let v = *c as i32 - zero;
        res = res * 10 + v;
    }
    res
}

pub fn is_digit(c: char) -> bool {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn lcmm(nums: &[i64]) -> i64 {
    let mut res = nums[0];
    for i in 1..nums.len() {
        res = lcm(res, nums[i]);
    }
    res
}
