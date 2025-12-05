use std::fs;

#[derive(Debug, Copy, Clone)]
struct Point {
    ih: usize,
    iw: usize,
    v: usize,
}

fn count_trailheads(map: &Vec<Vec<usize>>, zero_point: &Point) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut queue: Vec<Point> = vec![zero_point.clone()];
    let mut is_visited_nine_points: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let direction: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut num_trailhead = 0;

    while let Some(current_point) = queue.pop() {
        for &(dh, dw) in &direction {
            let next_ih = current_point.ih as isize + dh;
            let next_iw = current_point.iw as isize + dw;

            if next_ih >= 0 && next_ih < height as isize && next_iw >= 0 && next_iw < width as isize
            {
                let next_value = map[next_ih as usize][next_iw as usize];
                if next_value.saturating_sub(current_point.v) == 1
                    && !is_visited_nine_points[next_ih as usize][next_iw as usize]
                {
                    if next_value == 9 {
                        num_trailhead += 1;
                        is_visited_nine_points[next_ih as usize][next_iw as usize] = true;
                    }
                    let next_point = Point {
                        ih: next_ih as usize,
                        iw: next_iw as usize,
                        v: next_value,
                    };
                    queue.push(next_point);
                }
            }
        }
    }
    return num_trailhead;
}
fn main() {
    let path = "src/bin/input/day10.txt";

    let mx: Vec<Vec<usize>> = fs::read_to_string(path)
        .expect("Error when reading file")
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Error when parsing number") as usize)
                .collect()
        })
        .collect();

    let mut sum_num_trailheads = 0;

    for ih in 0..mx.len() {
        for iw in 0..mx[0].len() {
            if mx[ih][iw] == 0 {
                let num_trailheads = count_trailheads(
                    &mx,
                    &Point {
                        ih: ih,
                        iw: iw,
                        v: mx[ih][iw],
                    },
                );
                println!("{}", num_trailheads);
                sum_num_trailheads += num_trailheads;
            }
        }
    }

    println!("{:?}", sum_num_trailheads);
}
