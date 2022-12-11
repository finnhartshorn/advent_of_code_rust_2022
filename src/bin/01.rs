use itertools::Itertools;
use std::cmp;
use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .scan([0, 0], |state, line| {
                if line.is_empty() {
                    state[1] += line.parse::<u32>().unwrap();
                } else {
                    state[0] = cmp::max(state[0], state[1]);
                    state[1] = 0;
                }
                Some([state[0], state[1]])
            })
            .last()
            .unwrap()[0],
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| s.parse::<u32>())
            .coalesce(|x, y| {
                if (x.is_err()) || (y.is_err()) {
                    Err((x, y))
                } else {
                    Ok(Ok(x.unwrap() + y.unwrap()))
                }
            })
            .filter_map(Result::ok)
            .scan(BinaryHeap::new(), |state, line| {
                state.push(line);
                if state.len() >= 3 {
                    let mut state_2 = state.clone();
                    Some([
                        state_2.pop().unwrap(),
                        state_2.pop().unwrap(),
                        state_2.pop().unwrap(),
                    ])
                } else {
                    Some([0, 0, 0])
                }
            })
            .last()
            .unwrap()
            .into_iter()
            .sum(),
    )
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
