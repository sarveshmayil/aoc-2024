advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        re.captures_iter(input)
            .map(|cap| {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();

                a * b
            })
            .sum()
        )
}

pub fn part_two(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;

    Some(
        re.captures_iter(input)
            .map(|cap| {
                if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
                    if enabled {
                        let a = a.as_str().parse::<i32>().unwrap();
                        let b = b.as_str().parse::<i32>().unwrap();

                        a * b
                    } else {
                        0
                    }
                } else {
                    match &cap[0] {
                        "don't()" => enabled = false,
                        "do()" => enabled = true,
                        _ => {}
                    }
                    0
                }
            })
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}