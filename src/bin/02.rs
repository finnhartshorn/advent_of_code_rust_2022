pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let mut splitter = line.split_whitespace().map(|n| n.chars().collect::<Vec<char>>()[0].into());
        let opponent: u32 = splitter.next().unwrap();
        let me: u32 = splitter.next().unwrap();
        match me - opponent {
            23 => me + 3 - 87,
            24 | 21 => me + 6 - 87,
            _ => me - 87,
        }
    }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let mut splitter = line.split_whitespace().map(|n| Into::<u32>::into(n.chars().collect::<Vec<char>>()[0]));
        let opponent: u32 = splitter.next().unwrap() - 64;
        let me: u32 = splitter.next().unwrap() - 87;
        ((me + opponent) % 3) - 2 + (3 * me)
    }).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
