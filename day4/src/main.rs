use rayon::{iter::ParallelIterator, str::ParallelString};
use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("day4.txt").unwrap();
    let lines = input.par_lines().collect::<Vec<&str>>();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) -> usize {
    let r1 = Regex::new(r"(XMAS)").unwrap();
    let r2 = Regex::new(r"(SAMX)").unwrap();
    let mut occurences: usize = 0;

    for idx in 0..lines.len() {
        let line = lines[idx];

        occurences += r1.find_iter(line).count();
        occurences += r2.find_iter(line).count();
        occurences += line
            .par_char_indices()
            .map(|(i, c)| {
                if c != 'X' {
                    return 0;
                }

                let mut acc = 0;

                acc += search_part1(lines, idx, line, c, i);

                acc
            })
            .sum::<usize>();
    }

    println!("{occurences}");

    occurences
}

fn search_part1(lines: &[&str], line_idx: usize, line: &str, c: char, c_idx: usize) -> usize {
    let mut acc = 0usize;

    if line_idx >= 3 {
        let up3: Vec<char> = lines[line_idx - 3].chars().collect();
        let up2: Vec<char> = lines[line_idx - 2].chars().collect();
        let up1: Vec<char> = lines[line_idx - 1].chars().collect();

        if c == 'X' && up1[c_idx] == 'M' && up2[c_idx] == 'A' && up3[c_idx] == 'S' {
            acc += 1;
        }

        if c_idx >= 3
            && c == 'X'
            && up1[c_idx - 1] == 'M'
            && up2[c_idx - 2] == 'A'
            && up3[c_idx - 3] == 'S'
        {
            acc += 1;
        }

        if c_idx < (line.len() - 3)
            && c == 'X'
            && up1[c_idx + 1] == 'M'
            && up2[c_idx + 2] == 'A'
            && up3[c_idx + 3] == 'S'
        {
            acc += 1;
        }
    }
    if line_idx < (lines.len() - 3) {
        let down1: Vec<char> = lines[line_idx + 1].chars().collect();
        let down2: Vec<char> = lines[line_idx + 2].chars().collect();
        let down3: Vec<char> = lines[line_idx + 3].chars().collect();

        if c == 'X' && down1[c_idx] == 'M' && down2[c_idx] == 'A' && down3[c_idx] == 'S' {
            acc += 1;
        }

        if c_idx >= 3
            && c == 'X'
            && down1[c_idx - 1] == 'M'
            && down2[c_idx - 2] == 'A'
            && down3[c_idx - 3] == 'S'
        {
            acc += 1;
        }

        if c_idx < (line.len() - 3)
            && c == 'X'
            && down1[c_idx + 1] == 'M'
            && down2[c_idx + 2] == 'A'
            && down3[c_idx + 3] == 'S'
        {
            acc += 1;
        }
    }

    acc
}

fn part2(lines: &[&str]) -> usize {
    let mut occurences = 0usize;
    for idx in 1..lines.len() - 1 {
        let line = lines[idx];
        occurences += line
            .char_indices()
            .map(|(c_idx, c)| {
                if c == 'A' {
                    search_part2(lines, idx, line, c, c_idx)
                } else {
                    0
                }
            })
            .sum::<usize>()
    }

    println!("{occurences}");

    occurences
}

fn search_part2(lines: &[&str], line_idx: usize, line: &str, c: char, c_idx: usize) -> usize {
    let up: &str = lines[line_idx - 1];
    let down: &str = lines[line_idx + 1];

    if c_idx == 0 || c_idx == line.len() - 1 || c != 'A' {
        return 0;
    }

    let upper = up
        .get((c_idx - 1)..=(c_idx + 1))
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let lower = down
        .get((c_idx - 1)..=(c_idx + 1))
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    match (upper[0], upper[2], lower[0], lower[2]) {
        | ('M','S','M','S')
        | ('S','M','S','M')
        | ('S', 'S', 'M', 'M')
        | ('M', 'M', 'S', 'S') => 1,
        _ => 0,
    }
}