use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut dir_sizes = HashMap::new();
    let mut dir_stack: Vec<String> = Vec::new();
    input.lines().for_each(|command| {
        let mut words = command.split_whitespace();
        match words.next().unwrap() {
            "$" => match words.next().unwrap() {
                "cd" => match words.next().unwrap() {
                    ".." => {
                        dir_stack.pop();
                    }
                    directory => {
                        if dir_stack.is_empty() {
                            dir_stack.push(directory.to_string());
                        } else {
                            let mut parent: String = dir_stack.clone().last().unwrap().clone();
                            parent.push('/');
                            parent.push_str(directory);
                            dir_stack.push(parent);
                        }
                    }
                },
                "ls" => {}
                _ => {}
            },
            "dir" => {}
            number => {
                let size = number.parse::<u32>().unwrap();
                for i in 0..dir_stack.len() {
                    dir_sizes
                        .entry(dir_stack.clone()[i].clone())
                        .and_modify(|size_sum| *size_sum += size)
                        .or_insert(size);
                }
            }
        }
    });
    Some(
        dir_sizes
            .values()
            .cloned()
            .filter(|size| *size <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dir_sizes = HashMap::new();
    let mut dir_stack: Vec<String> = Vec::new();
    let mut total_size = 0;
    input.lines().for_each(|command| {
        let mut words = command.split_whitespace();
        match words.next().unwrap() {
            "$" => match words.next().unwrap() {
                "cd" => match words.next().unwrap() {
                    ".." => {
                        dir_stack.pop();
                    }
                    directory => {
                        if dir_stack.is_empty() {
                            dir_stack.push(directory.to_string());
                        } else {
                            let mut parent: String = dir_stack.clone().last().unwrap().clone();
                            parent.push('/');
                            parent.push_str(directory);
                            dir_stack.push(parent);
                        }
                    }
                },
                "ls" => {}
                _ => {}
            },
            "dir" => {}
            number => {
                let size = number.parse::<u32>().unwrap();
                for i in 0..dir_stack.len() {
                    dir_sizes
                        .entry(dir_stack.clone()[i].clone())
                        .and_modify(|size_sum| *size_sum += size)
                        .or_insert(size);
                }
                total_size += size;
            }
        }
    });
    let min_size = 30000000 - (70000000 - total_size);
    dir_sizes
        .values()
        .cloned()
        .filter(|size| *size >= min_size)
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
