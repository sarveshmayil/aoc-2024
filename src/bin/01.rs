advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    Some(
        left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) = (Vec::new(), Vec::new());
    
    input
        .lines()
        .map(|line| line.split_whitespace().filter_map(|n| n.parse::<i32>().ok()))
        .for_each(|mut nums| {
            left.push(nums.next().unwrap());
            right.push(nums.next().unwrap());
        });

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}