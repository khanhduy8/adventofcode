use std::fs;

fn run_equation(operand: (isize, isize), operator: &str) -> Option<isize> {
    match operator {
        "*" => Some(operand.0 * operand.1),
        "+" => Some(operand.0 + operand.1),
        "||" => Some(
            format!("{}{}", operand.0, operand.1)
                .parse::<isize>()
                .expect("Error when concat 2 numbers"),
        ),
        &_ => None,
    }
}

fn build_equation_tree(operands: &Vec<isize>) -> Vec<(isize, isize)> {
    let num_operands = operands.len();
    let num_nodes: usize = 3_usize.pow(num_operands as u32) - 1;
    let mut equation_tree: Vec<(isize, isize)> = vec![(0, 0); num_nodes];
    equation_tree[0] = (0, operands[0]);

    for idx in 0..equation_tree.len() {
        let (level, operand_1) = equation_tree[idx];

        if level >= num_operands as isize - 1 {
            break;
        }

        if let (Some(left_node), Some(mid_node), Some(right_node)) = (
            run_equation((operand_1, operands[level as usize + 1]), "*"),
            run_equation((operand_1, operands[level as usize + 1]), "||"),
            run_equation((operand_1, operands[level as usize + 1]), "+"),
        ) {
            equation_tree[idx * 3 + 1] = (level + 1, left_node);
            equation_tree[idx * 3 + 2] = (level + 1, mid_node);
            equation_tree[idx * 3 + 3] = (level + 1, right_node);
        };
    }

    return equation_tree;
}

fn main() {
    let path = "src/bin/input/day7.txt";
    let lines: Vec<(isize, Vec<isize>)> = fs::read_to_string(path)
        .expect("error while reading file")
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
            let test_value = parts[0].parse::<isize>().expect("Error test value");
            let values = parts[1]
                .split_whitespace()
                .map(|s| s.trim().parse::<isize>().expect("Error values"))
                .collect::<Vec<isize>>();
            (test_value, values)
        })
        .collect();

    // println!("{:?}", lines);

    let mut sum_of_test_value = 0;

    for idx in 0..lines.len() {
        let (test_value, values) = &lines[idx];
        let equation_tree = build_equation_tree(&values);

        for (level, equation_result) in equation_tree {
            if level == values.len() as isize - 1 && equation_result == *test_value {
                // println!("{} {}", level, equation_result);
                sum_of_test_value += *test_value;
                break;
            }
        }
    }

    println!("{}", sum_of_test_value);
}
