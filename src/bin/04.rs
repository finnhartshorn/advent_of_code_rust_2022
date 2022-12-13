pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let values = line
                    .split(',')
                    .flat_map(|line| line.split('-').map(|digit| digit.parse::<u32>().unwrap()))
                    .collect::<Vec<u32>>();
                ((values[0] >= values[2] && values[1] <= values[3])
                    || (values[0] <= values[2] && values[1] >= values[3])) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let values = line
                    .split(',')
                    .flat_map(|line| line.split('-').map(|digit| digit.parse::<u32>().unwrap()))
                    .collect::<Vec<u32>>();
                !((values[0] > values[3] && values[1] > values[3])
                    || (values[0] < values[2] && values[1] < values[2])) as u32
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
