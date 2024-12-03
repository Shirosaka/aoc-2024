use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};
use regex::Regex;

fn parse(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn part1(input: String) -> i32 {
    let regex = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let lines = input.par_lines();
    let res = lines
        .fold(
            || 0,
            |acc, line| {
                let matches: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();

                let values: Vec<Vec<i32>> = matches
                    .par_iter()
                    .map(|ma| {
                        ma.replace("mul(", "")
                            .replace(")", "")
                            .split(",")
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    })
                    .collect();

                let sum: i32 = values
                    .par_iter()
                    .fold(
                        || 0,
                        |acc2, x| {
                            if x.len() > 2 {
                                panic!("len invalid")
                            }
                            acc2 + (x[0] * x[1])
                        },
                    )
                    .sum();

                acc + sum
            },
        )
        .sum();

    res
}

fn part2(input: String) -> i32 {
    let regex = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let lines = input.lines();
    let mut enabled = true;
    let res = lines.fold(0, |acc, line| {
        let matches: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();
        let mut sum = 0i32;

        for ma in matches {
            match ma {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if !enabled {
                        continue;
                    }

                    let nums = ma
                        .replace("mul(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    sum += nums[0] * nums[1];
                }
            }
        }

        acc + sum
    });

    res
}

fn main() {
    let input = parse("day3.txt");
    let result = part1(input.clone());
    println!("Result:{result}");
    let result2 = part2(input.clone());
    println!("Result2:{result2}");
}
