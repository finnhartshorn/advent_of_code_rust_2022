use std::cmp;
use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
        .split("\n\n")
            .scan(0, |state: &mut u32, line_group| {
                Some(cmp::max::<u32>(line_group.lines().map(|line| line.parse::<u32>().unwrap()).sum::<u32>(), state.clone()))
            })
            .last()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bin_heap = input.split("\n\n")
    .map(|line_group| {
        line_group.lines().map(|line| line.parse::<u32>().unwrap()).sum::<u32>()
    }).collect::<BinaryHeap<u32>>();
    Some([bin_heap.pop().unwrap(), bin_heap.pop().unwrap(), bin_heap.pop().unwrap()].into_iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
