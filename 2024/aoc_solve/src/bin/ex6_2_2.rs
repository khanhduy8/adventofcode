use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_pos(&self, pos: &(isize, isize)) -> (isize, isize) {
        let &(h, w) = pos;
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
) -> (bool, Option<Vec<((isize, isize), Direction)>>) {
    let height = lines.len();
    let width = lines[0].len();

    let mut trace_routes: Vec<Vec<(bool, Option<Direction>)>> =
        vec![vec![(false, None); width]; height];
    let mut routes: Vec<((isize, isize), Direction)> = vec![];

    let mut cur_pos = *start_pos;
    let mut cur_dir = *start_dir;
    let mut _step: usize = 0;

    routes.push((cur_pos, cur_dir));

    while cur_pos.0 >= 0
        && cur_pos.0 < height as isize
        && cur_pos.1 >= 0
        && cur_pos.1 < width as isize
    {
        let mut next_pos = cur_dir.move_pos(&cur_pos);
        trace_routes[cur_pos.0 as usize][cur_pos.1 as usize] = (true, Some(cur_dir));

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
            next_pos = cur_dir.move_pos(&cur_pos);
            //Check turn twice because case
            //#.
            //^#
            next_step_value = lines[next_pos.0 as usize][next_pos.1 as usize];
            if next_step_value == '#' {
                cur_dir = cur_dir.change_dir();
                next_pos = cur_dir.move_pos(&cur_pos);
            }

            cur_pos = next_pos;
            _step += 1;
        } else {
            cur_pos = next_pos;
            _step += 1;
        }

        if let (_is_travel, Some(prev_dir)) = trace_routes[next_pos.0 as usize][next_pos.1 as usize]
        {
            if cur_dir == prev_dir {
                return (true, Some(routes));
            }
        }
        // println!("{:?} {:?}", cur_pos, cur_dir);
        routes.push((cur_pos, cur_dir));
    }

    return (false, Some(routes));
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

    let mut loop_route_obstruction: HashSet<(isize, isize)> = HashSet::new();

    if let (_, Some(routes)) = check_is_loop(&lines, &start_pos, &start_dir) {
        let distinct_routes: HashSet<(isize, isize)> = routes.iter().map(|&(pos, _)| pos).collect();

        println!("{}", distinct_routes.len());

        for (route_pos, route_dir) in routes {
            let mut next_pos = route_dir.move_pos(&route_pos);
            let mut clone_map = lines.clone();

            if next_pos.0 > 0
                && next_pos.0 < height as isize
                && next_pos.1 >= 0
                && next_pos.1 < width as isize
            {
                let next_value = clone_map[next_pos.0 as usize][next_pos.1 as usize];
                if next_value == '.' {
                    clone_map[next_pos.0 as usize][next_pos.1 as usize] = '#';
                } else if next_value == '#' {
                    next_pos = route_dir.change_dir().move_pos(&route_pos);
                    clone_map[next_pos.0 as usize][next_pos.1 as usize] = '#';
                }
            } else {
                continue;
            }

            if let (is_loop, Some(_loop_routes)) = check_is_loop(&clone_map, &start_pos, &start_dir)
            {
                if is_loop {
                    // println!("Loop: {:?} {:?}", route_pos, next_pos);
                    loop_route_obstruction.insert(next_pos);
                }
            }
        }
    };

    println!("{}", loop_route_obstruction.len());
}
