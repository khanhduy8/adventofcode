use std::{collections::HashMap, fs};

fn sort_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Option<(bool, Option<i32>)> {
    let mut modified_update = update.clone();
    let mut is_valid = true;

    for i in 0..modified_update.len() {
        for j in i + 1..modified_update.len() {
            let page_before = modified_update[i];
            let page_after = modified_update[j];

            if let Some(rule_pages) = rules.get(&page_after) {
                if rule_pages.contains(&page_before) {
                    modified_update.swap(i, j);
                    is_valid = false;
                }
            }
        }
    }

    println!("{:?}", modified_update);
    return Some((
        is_valid,
        modified_update.get(modified_update.len() / 2).copied(),
    ));
}
fn main() {
    let path = "src/bin/input/day5.txt";
    let lines: Vec<String> = fs::read_to_string(path)
        .expect("error while reading file")
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut is_reading_section1 = true;

    for line in lines {
        if line.trim() == "" {
            is_reading_section1 = false;
            continue;
        }

        if is_reading_section1 {
            let parts: Vec<&str> = line.trim().split("|").map(|s| s.trim()).collect();
            let first_param_rule = parts[0]
                .parse::<i32>()
                .expect("Error when parse page number");
            let second_param_rule = parts[1]
                .parse::<i32>()
                .expect("Error when parse page number");

            rules
                .entry(first_param_rule)
                .or_insert_with(Vec::new)
                .push(second_param_rule);
        } else {
            let update: Vec<i32> = line
                .split(",")
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            updates.push(update);
        }
    }

    let mut total_num_mid_pages = 0;
    for update in updates {
        if let Some((is_valid, Some(mid_page))) = sort_update(&update, &rules) {
            if !is_valid {
                total_num_mid_pages += mid_page;
            }
        }
    }

    println!("{}", total_num_mid_pages);
}
