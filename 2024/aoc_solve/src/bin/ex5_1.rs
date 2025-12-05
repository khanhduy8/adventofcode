use std::{collections::HashMap, fs};

fn check_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    for (i, &page_before) in update.iter().enumerate() {
        for j in i + 1..update.len() {
            let page_after = update[j];

            if let Some(rule_pages) = rules.get(&page_after) {
                if rule_pages.contains(&page_before) {
                    return None;
                }
            }
        }
    }
    return update.get(update.len() / 2).copied();
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
        if let Some(mid_page) = check_update(&update, &rules) {
            // println!("{}, {:?}", mid_page, update);
            total_num_mid_pages += mid_page;
        }
    }

    println!("{}", total_num_mid_pages);
}
