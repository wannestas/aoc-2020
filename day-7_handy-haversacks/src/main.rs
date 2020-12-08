use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let raw_input = include_str!("../input.txt");

    let input = raw_input
        .lines()
        .map(|x| {
            let first_split = x.trim_end_matches('.');
            let mut first_split = first_split.split("contain");
            let mut key = first_split.next().unwrap().split_whitespace().collect::<String>();
            key = String::from(key.trim_end_matches('s'));
            let value: HashMap<String, usize> = first_split
                .next()
                .unwrap()
                .split(',')
                .map(|y| {
                    let mut split = y.split_whitespace();
                    let value = { match split.next().unwrap() {
                        "no" => 0,
                        z => z.parse().unwrap(),
                    }};
                    let mut key = split.collect::<String>();
                    key = String::from(key.trim_end_matches('s'));
                    (key, value)
                })
                .collect();
            (key, value)
        })
        .collect::<HashMap<String, HashMap<String, usize>>>();

    let start = Instant::now();
    println!("Part 1 solution: {}", part_1(&input));
    println!("Finished aftr {:?} seconds", start.elapsed());

    let start = Instant::now();
    println!("Part 2 solution: {}", part_2(&input));
    println!("Finished aftr {:?} seconds", start.elapsed());
}

fn part_1(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut counter = 0;
    for i in input.keys() {
        if contains(input, i, "shinygoldbag") {
            counter += 1;
        }
    }
    counter
}

fn part_2(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    count(input, "shinygoldbag")
}

fn contains(input: &HashMap<String, HashMap<String, usize>>, bag_1: &str, bag_2: &str) -> bool {
    if input[bag_1].keys().any(|x| x == &bag_2) || {
        let mut ret = false;
        for i in input[bag_1].keys() {
            if i != "otherbag" && contains(input, i, bag_2) {
                ret = true
            }
        }
        ret
    }
    {
        return true
    }
    false
}

fn count(input: &HashMap<String, HashMap<String, usize>>, bag: &str) -> usize {
    let mut counter = 0;
    for i in input[bag].keys() {
        if i != "otherbag" {
            counter += input[bag][i] * (count(input, i) + 1);
        }
    }
    //println!("counter: {}", counter);
    counter
}
