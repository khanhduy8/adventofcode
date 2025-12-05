use core::num;
use std::{collections::HashMap, fs, vec};

fn print_fence(perimeters: &Vec<Vec<HashMap<char, usize>>>) {
    for i in 0..perimeters.len() {
        for j in 0..perimeters[0].len() {
            if i % 2 == 0 && j % 2 == 0 {
                print!("+");
            } else if i % 2 == 1 && j % 2 == 1 {
                if let Some((&key, _)) = perimeters[i][j].iter().find(|(_, &v)| v == 0) {
                    print!("{}", key)
                };
            } else if perimeters[i][j].iter().filter(|(_, &v)| v == 1).count() != 0 {
                if i % 2 == 1 {
                    print!("|");
                } else if j % 2 == 1 {
                    print!("-");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn calc_price_area(garden: &Vec<Vec<char>>) {
    let height = garden.len();
    let width = garden[0].len();
    let direction: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut zone_checked_map: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut zone_counter: HashMap<usize, usize> = HashMap::new();
    let mut zone_fence_counter: HashMap<usize, usize> = HashMap::new();

    let mut zone_id = 0;

    for ih in 0..height {
        for iw in 0..width {
            if zone_checked_map[ih][iw] {
                continue;
            }

            let tree_type = garden[ih][iw];
            let mut queue: Vec<(usize, usize)> = vec![];
            zone_checked_map[ih][iw] = true;
            *zone_counter.entry(zone_id).or_insert(0) += 1;
            queue.push((ih, iw));

            while let Some((ch, cw)) = queue.pop() {
                for &(dh, dw) in &direction {
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
                            queue.push((next_h as usize, next_w as usize));
                            *zone_counter.entry(zone_id).or_insert(0) += 1;
                            zone_checked_map[next_h as usize][next_w as usize] = true;
                        } else if next_tree_type != tree_type {
                            *zone_fence_counter.entry(zone_id).or_insert(0) += 1;
                        }
                    } else {
                        *zone_fence_counter.entry(zone_id).or_insert(0) += 1;
                    }
                }
            }
            zone_id += 1;
        }
    }

    let mut price = 0;
    let mut num_zone = 0;

    for (k, v) in &zone_counter {
        num_zone += 1;
        price += *v * zone_fence_counter[k];
    }

    // println!("{:?}", zone_counter);
    // println!("{:?}", zone_fence_counter);
    println!("{} {}", price, num_zone);
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

    print_fence(&perimeters);
    calc_price_area(&garden);
}
