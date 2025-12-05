use std::fs;

#[derive(Clone, Copy, Debug)]
struct Coordinate(isize, isize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Antennas,
    Antinode,
    Dot,
}

fn add_inbound_antinode(antinode: Coordinate, antinodes_map: &mut Vec<Vec<Type>>) -> bool {
    let height = antinodes_map.len();
    let width = antinodes_map[0].len();

    if antinode.0 < 0
        || antinode.0 >= height as isize
        || antinode.1 < 0
        || antinode.1 >= width as isize
    {
        return false;
    } else {
        antinodes_map[antinode.0 as usize][antinode.1 as usize] = Type::Antinode;
    }

    return true;
}

fn count_antinodes(map: &Vec<Vec<char>>) -> usize {
    let heigh = map.len();
    let width = map[0].len();

    let mut antennas: Vec<Coordinate> = vec![];
    let mut antinodes_map: Vec<Vec<Type>> = vec![vec![Type::Dot; width]; heigh];

    for h in 0..heigh {
        for w in 0..width {
            let map_value = map[h][w];

            if map_value != '.' {
                antennas.push(Coordinate(h as isize, w as isize));
                antinodes_map[h][w] = Type::Antennas;
            }
        }
    }

    let mut num_antinodes: usize = 0;

    for i in 0..antennas.len() {
        for j in (i + 1)..antennas.len() {
            let antenna_1 = antennas[i];
            let antenna_2 = antennas[j];

            if map[antenna_1.0 as usize][antenna_1.1 as usize]
                == map[antenna_2.0 as usize][antenna_2.1 as usize]
            {
                antinodes_map[antenna_1.0 as usize][antenna_1.1 as usize] = Type::Antinode;
                antinodes_map[antenna_2.0 as usize][antenna_2.1 as usize] = Type::Antinode;
            } else {
                continue;
            }

            let mut node_step = 1;

            while let true = add_inbound_antinode(
                Coordinate(
                    antenna_1.0 as isize + node_step * (antenna_1.0 - antenna_2.0) as isize,
                    antenna_1.1 as isize + node_step * (antenna_1.1 - antenna_2.1) as isize,
                ),
                &mut antinodes_map,
            ) {
                node_step += 1;
            }

            node_step = 1;

            while let true = add_inbound_antinode(
                Coordinate(
                    antenna_2.0 as isize + node_step * (antenna_2.0 - antenna_1.0) as isize,
                    antenna_2.1 as isize + node_step * (antenna_2.1 - antenna_1.1) as isize,
                ),
                &mut antinodes_map,
            ) {
                node_step += 1;
            }
        }
    }

    num_antinodes += antinodes_map
        .iter()
        .map(|row| row.iter().filter(|&&v| v == Type::Antinode).count())
        .sum::<usize>();

    return num_antinodes;
}
fn main() {
    let path = "src/bin/input/day8.txt";
    let lines = fs::read_to_string(path).expect("Error when read file");

    let map: Vec<Vec<char>> = lines
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();

    let num_antinodes = count_antinodes(&map);

    println!("{}", num_antinodes);
}
