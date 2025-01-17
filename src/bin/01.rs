advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input, true);
    Some(
        left
            // Iterate over left and right lists at the same time
            .iter()
            .zip(right.iter())
            // Calculate the absolute difference between the left and right values and sum them.
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    use std::collections::HashMap;

    // Use hashmaps to store the left and right values and their counts.
    let mut left_map: HashMap<i32, u32> = HashMap::new();
    let mut right_map: HashMap<i32, u32> = HashMap::new();

    for line in input.lines() {
        // Parse the line into a vector of integers.
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        // If the vector has two values, increment the counts of the left and right values in the hashmaps.
        if let [l, r] = nums[..] {
            *left_map.entry(l).or_insert(0) += 1;
            *right_map.entry(r).or_insert(0) += 1;
        }
    }

    Some(
        left_map
            // Iterate over the left values and their counts.
            .iter()
            // Multiply the left value by its count and the count of the corresponding right value.
            .map(|(l, l_count)| *l as u32 * l_count * right_map.get(l).unwrap_or(&0))
            // Sum the results.
            .sum()
    )
}

fn parse_input(input: &str, sort: bool) -> (Vec<i32>, Vec<i32>) {
    // Initialize two empty vectors to store the left and right list values.
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    
    input
        // Get iterator over lines in input.
        .lines()
        // For each line, split the line into whitespace-separated values and parse them as integers.
        .map(|line| line.split_whitespace().filter_map(|n| n.parse::<i32>().ok()))
        // For each line, get the first two values and push them to the left and right lists.
        .for_each(|mut nums| {
            left.push(nums.next().unwrap());
            right.push(nums.next().unwrap());
        });

    // If sort is true, sort the left and right lists.
    if sort {
        left.sort_unstable();
        right.sort_unstable();
    }

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

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}