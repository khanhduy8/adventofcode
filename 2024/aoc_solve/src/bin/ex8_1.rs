use std::fs;

#[derive(Clone, Copy, Debug)]
struct Coordinate(isize, isize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Antennas,
    Antinode,
    Dot,
}

fn add_inbound_antinode(antinode: Coordinate, mut antinodes_map: Vec<Vec<Type>>) -> Vec<Vec<Type>> {
    let height = antinodes_map.len();
    let width = antinodes_map[0].len();

    if antinode.0 < 0
        || antinode.0 >= height as isize
        || antinode.1 < 0
        || antinode.1 >= width as isize
    {
        return antinodes_map;
    } else {
        antinodes_map[antinode.0 as usize][antinode.1 as usize] = Type::Antinode;
    }

    return antinodes_map;
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

    for i in 0..antennas.len() {
        for j in (i + 1)..antennas.len() {
            let antenna_1 = antennas[i];
            let antenna_2 = antennas[j];

            if map[antenna_1.0 as usize][antenna_1.1 as usize]
                != map[antenna_2.0 as usize][antenna_2.1 as usize]
            {
                continue;
            }

            let antinode_1 = Coordinate(
                2 * antenna_1.0 as isize - antenna_2.0 as isize,
                2 * antenna_1.1 as isize - antenna_2.1 as isize,
            );
            let antinode_2 = Coordinate(
                2 * antenna_2.0 as isize - antenna_1.0 as isize,
                2 * antenna_2.1 as isize - antenna_1.1 as isize,
            );

            antinodes_map = add_inbound_antinode(antinode_1, antinodes_map);
            antinodes_map = add_inbound_antinode(antinode_2, antinodes_map);
        }
    }

    for row in &antinodes_map {
        println!("{:?}", row);
    }

    let num_antinodes: usize = antinodes_map
        .iter()
        .map(|row| row.iter().filter(|&&v| v == Type::Antinode).count())
        .sum();

    return num_antinodes;
}
fn main() {
    let path = "src/bin/input/day8.txt";
    let lines = fs::read_to_string(path).expect("Error when read file");

    let map: Vec<Vec<char>> = lines
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();

    println!("{:?}", map);

    let num_antinodes = count_antinodes(&map);

    println!("{}", num_antinodes);
}
