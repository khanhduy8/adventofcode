use std::fs;

fn blink_stones(arrangement: &mut Vec<usize>) {
    let mut times: usize = 25;
    let mut start = 0;

    while times > 0 {
        let end = arrangement.len();

        for i in start..end {
            let stone_value = arrangement[i];
            let number_of_digits_stone = format!("{}", stone_value).len();

            if stone_value == 0 {
                arrangement.push(1);
            } else if number_of_digits_stone % 2 == 0 {
                let stone_digits_string = format!("{}", stone_value);
                let (stone_1, stone_2) = stone_digits_string.split_at(number_of_digits_stone / 2);

                arrangement.push(stone_1.parse().expect("Error when parse stone 1"));
                arrangement.push(stone_2.parse().expect("Error when parse stone 2"));
            } else {
                arrangement.push(stone_value * 2024);
            }
        }
        times -= 1;

        if times == 0 && end <= arrangement.len() {
            for i in end..arrangement.len() {
                print!("{} ", arrangement[i])
            }
            println!("\n{}", arrangement.len() - end);
        }
        start = end;
    }

    // println!("{:?}", arrangement);
}

fn main() {
    let path = "src/bin/input/day11.txt";
    let init_arrangement: Vec<usize> = fs::read_to_string(path)
        .expect("Error when reading file")
        .split_whitespace()
        .map(|c| c.trim().parse::<usize>().expect("Error when parse number"))
        .collect();

    println!("{:?}", init_arrangement);
    let mut new_stones = init_arrangement.clone();
    blink_stones(&mut new_stones);
}
