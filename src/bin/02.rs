advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .iter()
            .map(safe1)
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    
    Some(
        reports
            .iter()
            .map(safe2)
            .sum()
    )
}

fn safe1(report: &Vec<i32>) -> u32 {
    // Establish if the report is increasing or decreasing based on the first two levels
    let increasing = report[0] < report[1];

    // Iterate over the report and check if each level change is safe
    let mut prev = report[0];
    for &level in &report[1..] {
        let diff = (prev - level).abs();

        if (diff < 1 || diff > 3) || (increasing && prev > level) || (!increasing && prev < level) {
            return 0;
        }

        prev = level;
    }

    1
}

fn safe2(report: &Vec<i32>) -> u32 {
    // If the report is safe as a whole, return 1
    if safe1(report) == 1 {
        return 1;
    }

    for remove_idx in 0..report.len() {
        // Create a subset of the report without the level at remove_idx
        let report_subset = report
            .iter()
            .enumerate()
            .filter_map(|(i, &level)| if i != remove_idx { Some(level) } else { None })
            .collect();

        if safe1(&report_subset) == 1 {
            return 1;
        }
    }

    0
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    
    input
        .lines()
        .for_each(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();
            reports.push(report);
        });
    
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}