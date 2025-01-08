advent_of_code::solution!(1);

// Solution idea:
// 1. Sort the two vectors
// 2. Create integer total_distance
// 3. Go through both vectors and add up the difference of the two items to totalDistance
pub fn part_one(input: &str) -> Option<u64> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines(){
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut total_distance: u64 = 0;

    for i in 0..left.len(){ // we only have to use one index as both vectors have the same size
        total_distance += (left[i] - right[i]).abs() as u64;
    }

    Some(total_distance)
}

// Solution idea:
// 1. Initialize similarity score to 0
// 2. Count the numbers of the right list with a hash map
// 3. Calculate the similarity score
use std::collections::HashMap;
pub fn part_two(input: &str) -> Option<u64> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines(){
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..left.len() {
        let count_right = right_map.entry(right[i]).or_insert(0);
        *count_right += 1;
    }

    let mut similarity_score: i32 = 0;

    for i in 0..left.len() {
        let key = left[i];
        if right_map.contains_key(&key) {
            similarity_score += left[i] * right_map.get(&key).unwrap();
        }
    }
    

    Some(similarity_score as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
