use std::process;

use five::{get_path, get_contents, split_contents, build_stacks, build_moves};

fn main() {
    let path = get_path().unwrap_or_else(|err| {
        println!("{}", err.to_string());
        process::exit(1);
    });

    let contents = get_contents(&path).unwrap_or_else(|err| {
        println!("{}", err.to_string());
        process::exit(1);
    });

    let splits = split_contents(&contents).unwrap_or_else(|err| {
        println!("{}", err.to_string());
        process::exit(1);
    });

    let (stack_string, move_string) = splits; 

    
    let mut stacks = build_stacks(&stack_string);

    let moves = build_moves(&move_string);

    println!("{:?}", stacks);
    println!("{:?}", moves);

}
