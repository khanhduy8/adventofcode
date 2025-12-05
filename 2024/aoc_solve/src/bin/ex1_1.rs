use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("src/bin/input/day1.txt").unwrap();
    let lines = input.lines();
    let number_of_lines: usize = lines.clone().count();
    let mut arr_a: Vec<i32> = vec![];
    let mut arr_b: Vec<i32> = vec![];

    for line in lines {
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();

        arr_a.push(a);
        arr_b.push(b);
    }

    arr_a.sort();
    arr_b.sort();

    let mut distance = 0;

    for i in 0..number_of_lines {
        let left = arr_a[i];
        let right = arr_b[i];
        let d = (left - right).abs();

        distance += (left - right).abs();
        println!("a: {} b: {} d: {} r: {}", left, right, d, distance);
    }

    println!("{}", distance);
}
