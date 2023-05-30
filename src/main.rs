use std::process;

use five::{
    build_moves, build_stacks, get_contents, get_path, process_part_a, process_part_b,
    split_contents,
};

fn main() {
    let path = get_path().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let contents = get_contents(&path).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let splits = split_contents(&contents).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let (stack_string, move_string) = splits;

    let mut stacks_a = build_stacks(&stack_string);
    let mut stacks_b = build_stacks(&stack_string);

    let moves = build_moves(&move_string);

    // println!("{:?}", &stacks);

    let answer_a = process_part_a(&mut stacks_a, &moves);

    println!("Part a answer is: {}", answer_a);

    let answer_b = process_part_b(&mut stacks_b, &moves);

    println!("Part b answer is: {}", answer_b);
}
