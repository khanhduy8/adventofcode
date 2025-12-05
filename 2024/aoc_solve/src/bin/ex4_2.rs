use std::fs;

fn main() {
    let path = "src/bin/input/day4.txt";
    let lines = fs::read_to_string(path).expect("error while reading file");

    let mx: Vec<String> = lines.split("\n").map(|line| line.to_string()).collect();

    let height: usize = mx.len();
    let width: usize = mx[0].len();
    let directions: Vec<(isize, isize)> = vec![(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut num_search_string = 0;

    for (ih, row) in mx.iter().enumerate() {
        for (iw, ch) in row.trim().chars().enumerate() {
            let mut line_1 = String::new();
            let mut line_2 = String::new();

            line_1.push(ch);
            line_2.push(ch);

            for d in 0..2 {
                let (dh, dw) = directions[d];

                let h = ih as isize + dh;
                let w = iw as isize + dw;

                if h >= 0 && h < height as isize && w >= 0 && w < width as isize {
                    if let Some(ch_get) = mx[h as usize].chars().nth(w as usize) {
                        line_1.push(ch_get);
                    }
                }
            }

            for d in 2..directions.len() {
                let (dh, dw) = directions[d];

                let h = ih as isize + dh;
                let w = iw as isize + dw;

                if h >= 0 && h < height as isize && w >= 0 && w < width as isize {
                    if let Some(ch_get) = mx[h as usize].chars().nth(w as usize) {
                        line_2.push(ch_get);
                    }
                }
            }

            if (line_1 == "AMS" || line_1 == "ASM") && (line_2 == "AMS" || line_2 == "ASM") {
                num_search_string += 1;
            }
        }
    }

    println!("{}", num_search_string);
}
