use std::collections::HashMap;

use crate::days::Days;

struct Day5Data {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<String>,
}

pub fn execute(day: &Days, real_data: bool) {
    let data = if real_data {
        parse(day.path())
    } else {
        parse(day.test_path())
    };

    let res = part1(&data);
    let res2 = part2(&data);

    println!("{res}");
    println!("{res2}");
}

fn parse(path: &str) -> Day5Data {
    let content = std::fs::read_to_string(path).unwrap();

    let rules = content
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let split = line
                .split("|")
                .map(String::from)
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (split[0], split[1])
        })
        .collect::<Vec<(i32, i32)>>();

    let updates = content
        .lines()
        .filter(|line| line.contains(","))
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut map = HashMap::<i32, Vec<i32>>::new();

    for rule in rules {
        map.entry(rule.0)
            .and_modify(|v| v.push(rule.1))
            .or_insert(vec![rule.1]);
    }

    Day5Data {
        rules: map,
        updates,
    }
}

fn part1(data: &Day5Data) -> i32 {
    let mut acc = 0;
    for update in &data.updates {
        let nums = update
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut valid = true;

        for num in &nums {
            if let Some(rules) = data.rules.get(num) {
                let pos = nums.iter().position(|n| n == num).unwrap();
                for rule in rules {
                    if let Some(rule_pos) = nums.iter().position(|n| n == rule) {
                        if rule_pos <= pos {
                            valid = false;
                        }
                    }
                }
            }
        }

        if valid {
            let middle = nums.len() / 2;
            acc += nums[middle];
        }
    }

    acc
}

fn part2(data: &Day5Data) -> i32 {
    let mut acc = 0;
    for update in &data.updates {
        let mut nums = update
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut valid = true;


        for _ in 0..100  {
            for (rule_num, rule) in data.rules.clone() {
                if let Some(pos) = nums.iter().position(|&n| n == rule_num) {
                    for rule_num in rule {
                        if let Some(rule_pos) = nums.iter().position(|&n| n == rule_num) {
                            if rule_pos <= pos {
                                valid = false;
                                nums.swap(pos, rule_pos);
                            }
                        }
                    }
                }
            }
        }

        if !valid {
            let middle = nums.len() / 2;
            acc += nums[middle];
        }
    }

    acc
}
