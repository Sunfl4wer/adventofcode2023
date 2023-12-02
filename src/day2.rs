use crate::utils;

pub fn part1() -> i32 {
    let content = utils::read_file("./data/day2.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut res = 0;
    for game in lines {
        if game == "" {
            continue;
        }
        let (valid_game, id) = possible(game.to_string());
        if valid_game {
            res += id;
        }
    }
    res
}

pub fn part2() -> i32 {
    let content = utils::read_file("./data/day2.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut res = 0;
    for game in lines {
        if game == "" {
            continue;
        }
        let ps = powersets(game.to_string());
        res += ps;
    }
    res
}

fn powersets(s: String) -> i32 {
    let mut no_red = 1;
    let mut no_blue = 1;
    let mut no_green = 1;

    let tokens: Vec<&str> = s.split(":").collect();
    let sets = tokens[1].split(";");
    for s in sets {
        let (nred, ngreen, nblue) = parse_set(s.to_string());
        if nred > no_red {
            no_red = nred;
        }
        if ngreen > no_green {
            no_green = ngreen;
        }
        if nblue > no_blue {
            no_blue = nblue;
        }
    }
    no_red * no_blue * no_green
}

fn possible(s: String) -> (bool, i32) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut no_red = 0;
    let mut no_blue = 0;
    let mut no_green = 0;

    let tokens: Vec<&str> = s.split(":").collect();
    let game_id: Vec<&str> = tokens[0].split(" ").collect();
    let id = utils::to_num(game_id[1].to_string());
    let sets = tokens[1].split(";");
    for s in sets {
        let (nred, ngreen, nblue) = parse_set(s.to_string());
        if nred > max_red || ngreen > max_green || nblue > max_blue {
            return (false, -1);
        }
    }
    (true, id)
}

fn parse_set(s: String) -> (i32, i32, i32) {
    let mut nred = 0;
    let mut ngreen = 0;
    let mut nblue = 0;
    let cubes = s.split(",");
    for cube in cubes {
        let c = cube.trim();
        let num_color: Vec<&str> = c.split(" ").collect();
        let num = num_color[0].to_string();
        let color = num_color[1];
        match color {
            "red" => {
                nred = utils::to_num(num);
            }
            "green" => {
                ngreen = utils::to_num(num);
            }
            "blue" => {
                nblue = utils::to_num(num);
            }
            _ => {
                continue;
            }
        }
    }
    (nred, ngreen, nblue)
}
