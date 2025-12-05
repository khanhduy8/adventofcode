use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_pos(&self, cur_pos: (isize, isize)) -> (isize, isize) {
        let (h, w) = cur_pos;
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
fn main() {
    let path = "src/bin/input/day6.txt";
    let lines: Vec<Vec<char>> = fs::read_to_string(path)
        .expect("error while reading file")
        .split("\n")
        .map(|line| line.trim().to_string())
        .map(|line| line.chars().collect())
        .collect();

    let height = lines.len();
    let width = lines[0].len();
    let mut mark_map: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let mut guard_chr = HashMap::new();
    guard_chr.insert('^', Direction::Up);
    guard_chr.insert('v', Direction::Down);
    guard_chr.insert('<', Direction::Left);
    guard_chr.insert('>', Direction::Right);

    let mut cur_pos: (isize, isize) = (0, 0);
    let mut cur_dir = Direction::Up;

    for h in 0..height {
        for w in 0..width {
            if let Some(&dir) = guard_chr.get(&lines[h][w]) {
                cur_pos = (h as isize, w as isize);
                cur_dir = dir;
                mark_map[cur_pos.0 as usize][cur_pos.1 as usize] = true;
            }
        }
    }

    let mut step: usize = 0;

    while cur_pos.0 >= 0
        && cur_pos.0 < height as isize
        && cur_pos.1 >= 0
        && cur_pos.1 < width as isize
    {
        let mut next_pos = cur_dir.move_pos(cur_pos);
        mark_map[cur_pos.0 as usize][cur_pos.1 as usize] = true;

        if next_pos.0 < 0
            || next_pos.0 >= height as isize
            || next_pos.1 < 0
            || next_pos.1 >= width as isize
        {
            break;
        }

        let mut next_step_value = lines[next_pos.0 as usize][next_pos.1 as usize];
        if next_step_value == '#' {
            cur_dir = cur_dir.change_dir();
            next_pos = cur_dir.move_pos(cur_pos);

            //Check turn twice because case
            //#.
            //^#
            next_step_value = lines[next_pos.0 as usize][next_pos.1 as usize];
            if next_step_value == '#' {
                cur_dir = cur_dir.change_dir();
                next_pos = cur_dir.move_pos(cur_pos);
            }

            cur_pos = next_pos;
            step += 1;
        } else {
            cur_pos = next_pos;
            step += 1;
        }

        println!("{:?} {:?} {}", cur_pos, cur_dir, step);
    }

    let distinct_pos: usize = mark_map
        .iter()
        .map(|h| h.iter().filter(|&&v| v).count())
        .sum();

    println!("{}", distinct_pos);
}
