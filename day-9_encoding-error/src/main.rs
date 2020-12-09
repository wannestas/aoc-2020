use std::time::Instant;

fn main() {
    let raw_input = include_str!("../input.txt");

    let input: Vec<usize> = raw_input.lines().map(|x| x.parse().unwrap()).collect();

    let start = Instant::now();
    let part_1_answer = part_1(&input);
    println!("The solution to part 1 was {}", input[part_1_answer]);
    println!("The solution was found in {:?}", start.elapsed());

    let start = Instant::now();
    println!(
        "The solution to part 2 was {}",
        part_2(&input, part_1_answer)
    );
    println!("The solution was found in {:?}", start.elapsed());

}

fn part_1(input: &Vec<usize>) -> usize {
    for i in 25..input.len() {
        if !find_sum(input, i) {
            return i;
        }
    }
    unreachable!()
}

fn part_2(input: &Vec<usize>, index: usize) -> usize {
    let sum_to = input[index];
    let input: Vec<&usize> = input.iter().filter(|x| **x < sum_to / 2).collect();
    for i in (0..input.len()).rev() {
        let sum_splice = sum_splice(&input, i, sum_to);
        if sum_splice != (0, 0) {
            let splice = &input[(sum_splice.0)..(sum_splice.1)];
            return **splice.iter().min().unwrap() + **splice.iter().max().unwrap();
        }
    }
    unreachable!()
}

fn find_sum(input: &Vec<usize>, number: usize) -> bool {
    let numbers_to_search_in = &input[number - 25..number];
    let number_to_search_for = input[number];
    for i in numbers_to_search_in {
        for j in numbers_to_search_in {
            if i == j {
                continue;
            }
            if i + j == number_to_search_for {
                return true;
            }
        }
    }
    false
}

fn sum_splice(input: &Vec<&usize>, index: usize, sum_to: usize) -> (usize, usize) {
    let mut new_index = index;
    let mut sum = 0;
    while sum < sum_to {
        sum += input[new_index];
        new_index -= 1;
    }
    new_index += 1;
    if sum == sum_to {
        return (new_index, index);
    }
    (0, 0)
}
