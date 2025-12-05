use std::{collections::HashMap, fs, vec};

fn main() {
    let input = fs::read_to_string("src/bin/input/day1.txt").unwrap();
    let lines = input.lines();
    let number_of_lines: usize = lines.clone().count();
    let mut arr_a: Vec<i32> = vec![];
    let mut map_b: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();

        arr_a.push(a);

        let counter = map_b.entry(b).or_insert(0);
        *counter += 1;
    }

    let mut distance = 0;

    for i in 0..number_of_lines {
        let a = arr_a[i];
        let mut occurence = 0;

        if map_b.contains_key(&a) {
            occurence = map_b[&a];
        }

        distance += a * occurence;
    }

    println!("{}", distance);
}
