fn parse(path: &str) -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string(path).unwrap();
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace();
            let left = nums.next().unwrap().parse::<i32>().unwrap();
            let right = nums.next().unwrap().parse::<i32>().unwrap();
            (left, right)
        })
        .collect();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();

    left.sort();
    right.sort();
    (left, right)
}

fn part1(left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.iter().fold(0, |acc, x| {
        acc + (x - right[left.iter().position(|e| e == x).unwrap()]).abs()
    })
}

fn part2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.iter().fold(0, |acc, x| {
        acc + (x * (right.iter().filter(|&r| r == x).count() as i32))
    })
}

fn main() {
    let (left, right) = parse("day1.txt");
    let distance = part1(left.clone(), right.clone());
    let similarity = part2(left.clone(), right.clone());
    println!("Distance:Similarity");
    println!("{distance}:{similarity}");
}