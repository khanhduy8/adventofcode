use std::{collections::HashMap, fs};

fn blink_stones(arrangement: &Vec<usize>, num_round: usize) -> usize {
    let mut stone_counter: HashMap<(usize, usize), usize> = HashMap::new();
    let mut sum_num_stone = 0;

    for &stone_value in arrangement {
        stone_counter.insert((0, stone_value), 1);
    }

    for i in 0..num_round {
        let mut updates: HashMap<(usize, usize), usize> = HashMap::new();
        for (&(round, stone_value), counter_num) in &stone_counter {
            let stone_digits = stone_value.to_string();
            if round != i {
                continue;
            }
            if stone_value == 0 {
                *updates.entry((round + 1, 1)).or_insert(0) += counter_num;
            } else if stone_digits.len() % 2 == 0 {
                let (part_0, part_1) = stone_digits.split_at(stone_digits.len() / 2);
                let num_0 = part_0
                    .parse::<usize>()
                    .expect("Error when parse splitted number");
                let num_1 = part_1
                    .parse::<usize>()
                    .expect("Error when parse splitted number");
                *updates.entry((round + 1, num_0)).or_insert(0) += counter_num;
                *updates.entry((round + 1, num_1)).or_insert(0) += counter_num;
            } else {
                *updates.entry((round + 1, stone_value * 2024)).or_insert(0) += counter_num;
            }
        }

        for ((round, stone_value), counter_num) in updates {
            *stone_counter.entry((round, stone_value)).or_insert(0) += counter_num;
        }
    }

    for ((round, _stone_value), counter_num) in stone_counter {
        if round == num_round {
            sum_num_stone += counter_num;
        }
    }
    return sum_num_stone;
}

fn main() {
    let path = "src/bin/input/day11.txt";
    let init_arrangement: Vec<usize> = fs::read_to_string(path)
        .expect("Error when reading file")
        .split_whitespace()
        .map(|c| c.trim().parse::<usize>().expect("Error when parse number"))
        .collect();

    println!("{:?}", init_arrangement);
    let result = blink_stones(&init_arrangement, 75);
    println!("{}", result);
}
