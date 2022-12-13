pub fn part_one(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut splitter = input.split("\n\n");
    let initial_stack_def = splitter.next().unwrap();
    let move_instructions = splitter.next().unwrap();

    let mut stack_def_lines = initial_stack_def.lines().rev();
    for _ in 0..((stack_def_lines.next().unwrap().len() + 1) / 4) {
        stacks.push(vec![]);
    }

    for line in stack_def_lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let pot_char = line.as_bytes()[1 + i * 4] as char;
            if pot_char != ' ' {
                stack.push(pot_char);
            }
        }
    }

    for line in move_instructions.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let number = words[1].parse::<u32>().unwrap();
        let from = words[3].parse::<u32>().unwrap() - 1;
        let to = words[5].parse::<u32>().unwrap() - 1;
        for _ in 0..number {
            let value = stacks[from as usize].pop().unwrap();
            stacks[to as usize].push(value);
        }
    }

    Some(
        stacks
            .iter_mut()
            .map(|stack| stack.pop().unwrap())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut splitter = input.split("\n\n");
    let initial_stack_def = splitter.next().unwrap();
    let move_instructions = splitter.next().unwrap();

    let mut stack_def_lines = initial_stack_def.lines().rev();
    for _ in 0..((stack_def_lines.next().unwrap().len() + 1) / 4) {
        stacks.push(vec![]);
    }

    for line in stack_def_lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let pot_char = line.as_bytes()[1 + i * 4] as char;
            if pot_char != ' ' {
                stack.push(pot_char);
            }
        }
    }

    for line in move_instructions.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let number = words[1].parse::<u32>().unwrap();
        let from = words[3].parse::<u32>().unwrap() - 1;
        let to = words[5].parse::<u32>().unwrap() - 1;
        let mut temp_stack = vec![];
        for _ in 0..number {
            let value = stacks[from as usize].pop().unwrap();
            temp_stack.push(value);
        }
        for _ in 0..number {
            let value = temp_stack.pop().unwrap();
            stacks[to as usize].push(value);
        }
    }

    Some(
        stacks
            .iter_mut()
            .map(|stack| stack.pop().unwrap())
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
