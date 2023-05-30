#![allow(dead_code)]
use std::collections::HashMap;
use std::{env, error::Error, fs};

pub fn get_path() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("File path not provided");
    }

    Ok(args.get(1).unwrap().to_string())
}

pub fn get_contents(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn split_contents(contents: &str) -> Result<(String, String), Box<dyn Error>> {
    let parts: Vec<&str> = contents.split("\n\n").collect();

    Ok((
        parts.first().unwrap().to_string(),
        parts.last().unwrap().to_string(),
    ))
}

pub fn build_stacks(input: &str) -> HashMap<char, Vec<char>> {
    let mut lines: Vec<&str> = input.lines().collect();

    let keys = lines.pop().unwrap();

    let mut stacks: HashMap<char, Vec<char>> = HashMap::new();

    let keys: Vec<(usize, char)> = keys
        .char_indices()
        .filter_map(|(idx, char)| {
            if matches!(char, '1'..='9') {
                stacks.insert(char, Vec::new());
                return Some((idx, char));
            };

            None
        })
        .collect();

    for line in lines {
        for key in &keys {
            let (index, char) = key;
            let stack = stacks.get_mut(char).unwrap();
            let char = line.chars().nth(*index).unwrap();
            match char {
                ' ' => (),
                _ => {
                    stack.insert(0, char);
                }
            }
        }
    }

    stacks
}

pub fn build_moves(input: &str) -> Vec<Instruction> {
    let moves: Vec<Instruction> = input.lines().map(convert_to_instruction).collect();

    moves
}

pub fn process_part_a<'a>(
    stacks: &'a mut HashMap<char, Vec<char>>,
    instructions: &'a Vec<Instruction>,
) -> String {
    for inst in instructions {
        let Instruction(count, from, to) = inst;

        for _ in 0..*count {
            let from_stack = stacks.get_mut(from).unwrap();
            let el: char = from_stack.pop().unwrap();
            let to_stack = stacks.get_mut(to).unwrap();
            to_stack.push(el);
        }
    }

    let mut results: Vec<(char, char)> = stacks
        .iter()
        .map(|(key, val)| {
            let last = *val.iter().last().unwrap();
            (*key, last)
        })
        .collect();

    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let results: Vec<char> = results.into_iter().map(|(_, value)| value).collect();

    String::from_iter(results)
}

pub fn process_part_b<'a>(
    stacks: &'a mut HashMap<char, Vec<char>>,
    instructions: &'a Vec<Instruction>,
) -> String {
    for inst in instructions {
        let Instruction(count, from, to) = inst;

        let count_size: usize = usize::try_from(*count).unwrap();
        let from_stack = stacks.get_mut(from).unwrap();
        let mut els: Vec<char> = from_stack.drain(from_stack.len() - count_size..).collect();
        let to_stack = stacks.get_mut(to).unwrap();
        to_stack.append(&mut els);
    }

    let mut results: Vec<(char, char)> = stacks
        .iter()
        .map(|(key, val)| {
            let last = *val.iter().last().unwrap();
            (*key, last)
        })
        .collect();

    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let results: Vec<char> = results.into_iter().map(|(_, value)| value).collect();

    String::from_iter(results)
}

fn convert_to_instruction(input: &str) -> Instruction {
    let splits: Vec<&str> = input.split(' ').collect();

    let count = splits[1];
    let from = splits[3].chars().next().unwrap();
    let to = splits[5].chars().next().unwrap();

    Instruction(count.parse().unwrap(), from, to)
}

#[derive(Debug)]
pub struct Instruction(u32, char, char);

pub fn char_reader(input: &char) -> bool {
    matches!(input, '1'..='9')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_reader_works() {
        assert!(char_reader(&'1'));
        assert!(char_reader(&'9'));
        assert!(!char_reader(&'0'));
        assert!(!char_reader(&' '));
    }
}
