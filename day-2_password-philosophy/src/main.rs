use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut input_raw = String::new();
    buf_reader.read_to_string(&mut input_raw)?;
    println!("{}", part1(&input_raw));
    println!("{}", part2(&input_raw));
    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let (min, max, character, value) = split(l);

            let count = value.chars().filter(|c| *c == character).count();

            return min <= count && count <= max;
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let (min, max, character, value) = split(l);

            let min_char = value.chars().nth(min-1).unwrap();
            let max_char = value.chars().nth(max-1).unwrap();

            return (min_char == character && max_char != character) || (max_char == character && min_char != character);
        }).count()
}

fn split(line: &str) -> (usize, usize, char, &str) {

    let parts: Vec<&str> = line.split_whitespace().collect();

    let range: Vec<usize> = parts[0]
        .split('-')
        .map(|c| c.parse().unwrap())
        .collect();
    let min = range[0];
    let max = range[1];

    let character = parts[1].chars().nth(0).unwrap();

    let value = parts[2];

    (min, max, character, value)
}
