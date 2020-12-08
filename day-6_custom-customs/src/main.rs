use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let raw_input = include_str!("../input.txt");

    let start = Instant::now();
    println!("Part 1 solution: {}", part_1(raw_input));
    println!("Finished aftr {:?} seconds", start.elapsed());

    let start = Instant::now();
    println!("Part 2 solution: {}", part_2(raw_input));
    println!("Finished aftr {:?} seconds", start.elapsed());
    println!("recompile plox uwU")
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x: &str| {
            x.chars()
                .filter(|y| y != &'\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x: &str| {
            let group_size = x.matches('\n').count() + 1;
            let first_participant = x.split_whitespace().collect::<Vec<&str>>()[0];
            let mut counter = 0;
            for i in first_participant.chars() {
                if x.matches(i).count() == group_size {
                    counter += 1
                }
            }
            counter
        })
        .sum()
}
