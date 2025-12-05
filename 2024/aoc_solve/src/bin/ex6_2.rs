use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_pos(&self, h: isize, w: isize) -> (isize, isize) {
        match self {
            Direction::Up => (h - 1, w),
            Direction::Down => (h + 1, w),
            Direction::Left => (h, w - 1),
            Direction::Right => (h, w + 1),
        }
    }

    fn change_dir(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn check_is_loop(
    lines: &Vec<Vec<char>>,
    start_pos: &(isize, isize),
    start_dir: &Direction,
) -> bool {
    let height = lines.len();
    let width = lines[0].len();

    let mut trace_routes: Vec<Vec<(bool, Option<Direction>)>> =
        vec![vec![(false, None); width]; height];

    let mut cur_pos = *start_pos;
    let mut cur_dir = *start_dir;

    trace_routes[cur_pos.0 as usize][cur_pos.1 as usize] = (true, Some(cur_dir));

    let mut _step: usize = 0;

    while cur_pos.0 >= 0
        && cur_pos.0 < height as isize
        && cur_pos.1 >= 0
        && cur_pos.1 < width as isize
    {
        let mut next_pos = cur_dir.move_pos(cur_pos.0, cur_pos.1);

        if next_pos.0 < 0
            || next_pos.0 >= height as isize
            || next_pos.1 < 0
            || next_pos.1 >= width as isize
        {
            break;
        }

        let next_step_value = lines[next_pos.0 as usize][next_pos.1 as usize];

        if next_step_value == '#' {
            cur_dir = cur_dir.change_dir();
            next_pos = cur_dir.move_pos(cur_pos.0, cur_pos.1);
            cur_pos = next_pos;
            _step += 1;
        } else {
            cur_pos = next_pos;
            _step += 1;
        }

        if let (is_travel, Some(prev_dir)) = trace_routes[cur_pos.0 as usize][cur_pos.1 as usize] {
            // println!("{:?} {:?}", cur_pos, cur_dir);
            if cur_dir == prev_dir {
                return true;
            }
        }
        trace_routes[cur_pos.0 as usize][cur_pos.1 as usize] = (true, Some(cur_dir));
    }

    return false;
}

fn main() {
    let path = "src/bin/input/day6.txt";
    let lines: Vec<Vec<char>> = fs::read_to_string(path)
        .expect("error while reading file")
        .split("\n")
        .map(|line| line.trim().to_string())
        .map(|line| line.chars().collect())
        .collect();

    // let mut count_new_obstruction: usize = 0;

    // println!("{}", count_new_obstruction);

    let height = lines.len();
    let width = lines[0].len();

    let mut guard_chr = HashMap::new();
    guard_chr.insert('^', Direction::Up);
    guard_chr.insert('v', Direction::Down);
    guard_chr.insert('<', Direction::Left);
    guard_chr.insert('>', Direction::Right);

    let mut start_pos: (isize, isize) = (0, 0);
    let mut start_dir = Direction::Up;

    for h in 0..height {
        for w in 0..width {
            if let Some(&dir) = guard_chr.get(&lines[h][w]) {
                start_pos = (h as isize, w as isize);
                start_dir = dir;
            }
        }
    }

    let mut count_loop: usize = 0;

    for h in 0..height {
        for w in 0..width {
            let mut try_map = lines.clone();
            let value = try_map[h][w];

            if value == '.' {
                // println!("Checking ({}, {})", h, w);
                try_map[h][w] = '#';
            }

            let is_loop = check_is_loop(&try_map, &start_pos, &start_dir);

            if is_loop {
                count_loop += 1;
                println!("({}, {}) {}", h, w, is_loop);
            }
        }
    }
    println!("{}", count_loop);
}
