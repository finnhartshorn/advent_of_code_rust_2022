use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (side_a, side_b) = line.split_at(line.len() / 2);
                let mut mask: u64 = 0;
                let mut mask_2: u64 = 0;
                let mut sum: u32 = 0;
                for char in side_a.chars() {
                    mask |= 1 << (char as u8 - b'A');
                }
                for char in side_b.chars() {
                    if ((mask & (1 << (char as u8 - b'A'))) != 0)
                        && ((mask_2 & (1 << (char as u8 - b'A'))) == 0)
                    {
                        if char as u8 >= b'a' {
                            sum += (char as u8 - b'a' + 1) as u32;
                        } else {
                            sum += (char as u8 - b'A' + 27) as u32;
                        }
                    }
                    mask_2 |= 1 << (char as u8 - b'A');
                }
                sum
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                let mut bit_mask: u64 = 4503599627370495;
                for line in chunk {
                    let mut line_bit_mask: u64 = 0;
                    for char in line.chars() {
                        let value = match char {
                            'a'..='z' => char as u8 - b'a',
                            'A'..='Z' => char as u8 - b'A' + 26,
                            _ => 0,
                        };
                        line_bit_mask |= 1u64 << value;
                    }
                    bit_mask &= line_bit_mask;
                }
                bit_mask.trailing_zeros() + 1
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
