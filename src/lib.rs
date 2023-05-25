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

pub fn build_stacks<'a>(input: &'a str) -> HashMap<char, Vec<&char>> {
    let mut lines: Vec<&str> = input.lines().collect();

    let keys = lines.pop().unwrap();

    let mut stacks: HashMap<char, Vec<&char>> = HashMap::new();

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

        for key in keys.clone() {
            let (index, char) = key;
            let stack = stacks.get_mut(&char).unwrap();
            let char = line.chars().nth(index).unwrap();
            stack.push(&char.clone());
        }
    }

    stacks
}

struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn build(count: &str, from: &str, to: &str) -> Instruction {
        Instruction {
            count: count.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    }
}

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
