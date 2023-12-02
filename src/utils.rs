use std::fs;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("should has string")
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
