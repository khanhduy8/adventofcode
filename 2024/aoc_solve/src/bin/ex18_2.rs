use std::{collections::VecDeque, fs, vec};

fn find_step(pos_bytes: &Vec<(usize, usize)>, byte_range: usize) -> isize {
    let heigh = 71;
    let width = 71;

    let mut mx: Vec<Vec<(isize, bool)>> = vec![vec![(0, false); width]; heigh];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));
    let direction: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (idx, (xb, yb)) in pos_bytes.iter().enumerate() {
        if idx < byte_range {
            mx[*yb][*xb] = (-1, true);
        }
    }

    while let Some((ix, iy)) = queue.pop_front() {
        let (current_step, _) = mx[iy][ix];
        mx[iy][ix].1 = true;

        for &(dx, dy) in &direction {
            let next_x = ix as isize + dx;
            let next_y = iy as isize + dy;

            if next_x < 0 || next_x >= width as isize || next_y < 0 || next_y >= heigh as isize {
                continue;
            }

            let (_, is_visited) = mx[next_y as usize][next_x as usize];

            if !is_visited {
                queue.push_back((next_x as usize, next_y as usize));
                mx[next_y as usize][next_x as usize] = (current_step + 1, true);
            }
        }
    }

    // for i in 0..7 {
    //     for j in 0..7 {
    //         let (v, _) = mx[i][j];
    //         if v == -1 {
    //             print!("  #");
    //         } else {
    //             print!("{:>3}", v);
    //         }
    //     }
    //     println!("");
    // }

    return mx[70][70].0;
}

fn main() {
    let path = "src/bin/input/day18.txt";
    let pos_bytes: Vec<(usize, usize)> = fs::read_to_string(path)
        .expect("error when reading file")
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim().split(",").collect();
            (
                parts[0]
                    .trim()
                    .parse::<usize>()
                    .expect("error when parse num"),
                parts[1]
                    .trim()
                    .parse::<usize>()
                    .expect("error when parse num"),
            )
        })
        .collect();
    println!("{:?}", pos_bytes);

    for i in 0..pos_bytes.len() {
        let v = find_step(&pos_bytes, i + 1);
        // println!("{}", v);
        if v == 0 {
            println!("{:?}", pos_bytes[i]);
            break;
        }
    }
}
