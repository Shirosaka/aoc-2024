fn parse(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.lines().map(str::to_string).collect::<Vec<String>>();
    lines
}

fn part1(lines: Vec<String>) -> i32 {
    let mut safe_reports = 0;

    for line in lines {
        let levels: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if levels[0] == levels[1] {
            continue;
        }

        let mut prev_level = levels[0];
        let increasing = prev_level < levels[1];
        let mut safe = true;

        for &level in levels.iter().skip(1) {
            if prev_level < level {
                if !increasing {
                    safe = false;
                    break;
                }

                let distance = level - prev_level;
                match distance {
                    1..=3 => (),
                    _ => {
                        safe = false;
                        break;
                    }
                }
            } else {
                if increasing {
                    safe = false;
                    break;
                }

                let distance = prev_level - level;
                match distance {
                    1..=3 => (),
                    _ => {
                        safe = false;
                        break;
                    }
                }
            }

            prev_level = level;
        }

        if safe {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
    safe_reports
}

fn part2(lines: Vec<String>) -> i32 {
    let levels = lines.iter().map(|x| {
        x.split_whitespace()
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut safe_levels = 0;

    levels.for_each(|level| {
        let mut prev_elem: Option<i32> = None;
        let mut ascending: Option<bool> = None;
        let mut bad_elements = 0;

        level.iter().for_each(|&elem| {
            if let Some(prev) = prev_elem {
                if prev == elem {
                    bad_elements += 1;
                    prev_elem = Some(elem);
                    return;
                }

                if let Some(ascended) = ascending {
                    if ascended && elem < prev || !ascended && prev < elem {
                        bad_elements += 1;
                        prev_elem = Some(elem);
                        return;
                    }
                } else {
                    ascending = Some(prev < elem)
                }

                let distance = (elem - prev).abs();

                match distance {
                    1..=3 => (),
                    _ => bad_elements += 1,
                };

                prev_elem = Some(elem);
            } else {
                // Set starting element as previous element and continue
                prev_elem = Some(elem);
            }
        });

        if bad_elements <= 1 {
            safe_levels += 1;
        }
    });

    println!("{safe_levels}");

    safe_levels
}

fn main() {
    let input = parse("day2.txt");
    part1(input.clone());
    part2(input.clone());
}