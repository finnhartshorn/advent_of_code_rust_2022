pub fn part_one(input: &str) -> Option<u32> {
    let mut tracking_array: [u8; 26] = [0; 26];
    let char_array: Vec<char> = input.chars().collect();
    let mut dupe_count = 0;

    for i in 0..char_array.len() {
        if tracking_array[char_array[i] as usize - 97] >= 1 {
            dupe_count += 1;
        } else if dupe_count == 0 && i > 2 {
            return Some(i as u32 + 1);
        }
        tracking_array[char_array[i] as usize - 97] += 1;
        if i > 2 {
            tracking_array[char_array[i - 3] as usize - 97] -= 1;
            if tracking_array[char_array[i - 3] as usize - 97] != 0 {
                dupe_count -= 1;
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tracking_array: [u8; 26] = [0; 26];
    let char_array: Vec<char> = input.chars().collect();
    let mut dupe_count = 0;

    for i in 0..char_array.len() {
        if tracking_array[char_array[i] as usize - 97] >= 1 {
            dupe_count += 1;
        } else if dupe_count == 0 && i > 12 {
            return Some(i as u32 + 1);
        }
        tracking_array[char_array[i] as usize - 97] += 1;
        if i > 12 {
            tracking_array[char_array[i - 13] as usize - 97] -= 1;
            if tracking_array[char_array[i - 13] as usize - 97] != 0 {
                dupe_count -= 1;
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
