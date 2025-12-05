use std::{
    collections::{HashMap, VecDeque},
    fs, vec,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_dir(&self) -> (isize, isize) {
        match &self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

fn count_side(project_side: &Vec<Vec<isize>>) -> usize {
    let mut num_side = 0;
    let num_layer = project_side.iter().map(|layer| layer.len()).max().unwrap();

    for layer_id in 0..num_layer {
        let mut num_side_each_layer = 0;
        let mut prev_v = -1;
        let mut side: Vec<isize> = vec![];
        for idx in 0..project_side.len() {
            let mut layer: Vec<isize> = project_side[idx].clone();

            if let Some(&v) = layer.get(layer_id) {
                side.push(v);
                if v == -1 {
                    prev_v = -1;
                    continue;
                }

                if v != prev_v {
                    prev_v = v;
                    num_side_each_layer += 1;
                }
            };
        }
        num_side += num_side_each_layer;
    }

    return num_side;
}

fn calc_price_area(garden: &Vec<Vec<char>>) {
    let height = garden.len();
    let width = garden[0].len();
    let direction: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut zone_checked_map: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut zone_counter: HashMap<usize, usize> = HashMap::new();
    let mut zone_fence_counter: HashMap<usize, usize> = HashMap::new();
    let mut project_left: HashMap<usize, Vec<Vec<isize>>> = HashMap::new(); //project_side[zone_id][idx_side][layer_id] = idx_otherside
    let mut project_right: HashMap<usize, Vec<Vec<isize>>> = HashMap::new();
    let mut project_up: HashMap<usize, Vec<Vec<isize>>> = HashMap::new();
    let mut project_down: HashMap<usize, Vec<Vec<isize>>> = HashMap::new();

    let mut zone_id = 0;

    for ih in 0..height {
        for iw in 0..width {
            if zone_checked_map[ih][iw] {
                continue;
            }

            let tree_type = garden[ih][iw];
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            zone_checked_map[ih][iw] = true;
            *zone_counter.entry(zone_id).or_insert(0) += 1;

            project_left.insert(zone_id, vec![vec![-1; width]; height]);
            project_right.insert(zone_id, vec![vec![-1; width]; height]);
            project_up.insert(zone_id, vec![vec![-1; height]; width]);
            project_down.insert(zone_id, vec![vec![-1; height]; width]);

            queue.push_back((ih, iw));

            while let Some((ch, cw)) = queue.pop_front() {
                for &dir in &direction {
                    let (dh, dw) = dir.get_dir();
                    let next_h = ch as isize + dh;
                    let next_w = cw as isize + dw;

                    if next_h >= 0
                        && next_h < height as isize
                        && next_w >= 0
                        && next_w < width as isize
                    {
                        let next_tree_type = garden[next_h as usize][next_w as usize];
                        if next_tree_type == tree_type
                            && !zone_checked_map[next_h as usize][next_w as usize]
                        {
                            queue.push_back((next_h as usize, next_w as usize));
                            *zone_counter.entry(zone_id).or_insert(0) += 1;
                            zone_checked_map[next_h as usize][next_w as usize] = true;
                        } else if next_tree_type != tree_type {
                            *zone_fence_counter.entry(zone_id).or_insert(0) += 1;
                            match dir {
                                Direction::Left => {
                                    if let Some(proj_left_vec) = project_left.get_mut(&zone_id) {
                                        proj_left_vec[ch][cw] = cw as isize;
                                    }
                                }
                                Direction::Down => {
                                    if let Some(proj_down_vec) = project_down.get_mut(&zone_id) {
                                        proj_down_vec[cw][ch] = ch as isize;
                                    }
                                }
                                Direction::Up => {
                                    if let Some(proj_up_vec) = project_up.get_mut(&zone_id) {
                                        proj_up_vec[cw][ch] = ch as isize;
                                    }
                                }
                                Direction::Right => {
                                    if let Some(proj_right_vec) = project_right.get_mut(&zone_id) {
                                        proj_right_vec[ch][cw] = cw as isize;
                                    }
                                }
                            }
                        }
                    } else {
                        *zone_fence_counter.entry(zone_id).or_insert(0) += 1;
                        match dir {
                            Direction::Left => {
                                if let Some(proj_left_vec) = project_left.get_mut(&zone_id) {
                                    proj_left_vec[ch][cw] = cw as isize;
                                }
                            }
                            Direction::Down => {
                                if let Some(proj_down_vec) = project_down.get_mut(&zone_id) {
                                    proj_down_vec[cw][ch] = ch as isize;
                                }
                            }
                            Direction::Up => {
                                if let Some(proj_up_vec) = project_up.get_mut(&zone_id) {
                                    proj_up_vec[cw][ch] = ch as isize;
                                }
                            }
                            Direction::Right => {
                                if let Some(proj_right_vec) = project_right.get_mut(&zone_id) {
                                    proj_right_vec[ch][cw] = cw as isize;
                                }
                            }
                        }
                    }
                }
            }
            zone_id += 1;
        }
    }

    let mut price = 0;
    let mut step = 0;

    for (k, v) in &zone_counter {
        step += 1;
        // println!("Checking zone {} have {} step: {}", k, v, step);
        let num_side = count_side(&project_left[&k])
            + count_side(&project_right[&k])
            + count_side(&project_up[&k])
            + count_side(&project_down[&k]);
        price += *v * num_side;
    }
    println!("{}", price);
}

fn main() {
    let path = "src/bin/input/day12.txt";
    let garden: Vec<Vec<char>> = fs::read_to_string(path)
        .expect("error when reading file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = garden.len();
    let width = garden[0].len();
    let direction: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    let mut perimeters: Vec<Vec<HashMap<char, usize>>> =
        vec![vec![HashMap::new(); width * 2 + 1]; height * 2 + 1];

    let mut step_h = 1;

    for h in 0..height {
        let mut step_w = 1;
        for w in 0..width {
            let tree_type: char = garden[h][w].clone();
            let h_center = h + step_h;
            let w_center = w + step_w;
            perimeters[h_center as usize][w_center as usize].insert(tree_type, 0);

            for &(dh, dw) in &direction {
                let h_fence = h_center as isize + dh;
                let w_fence = w_center as isize + dw;
                *perimeters[h_fence as usize][w_fence as usize]
                    .entry(tree_type)
                    .or_insert(0) += 1;
            }
            step_w += 1;
        }
        step_h += 1;
    }

    calc_price_area(&garden);
}
