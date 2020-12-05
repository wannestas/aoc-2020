use itertools::sorted;

fn main() {
    let raw_input = include_str!("../input.txt");
    let input: Vec<usize> = sorted(raw_input.split_whitespace().map(|x: &str| {
        let (mut row_min, mut row_max) = (1, 128);
        let (mut col_min, mut col_max) = (1, 8);
        for i in x.chars() {
            match i {
                'F' => row_max = (row_max + row_min) / 2,
                'B' => row_min = (row_max + row_min) / 2,
                'R' => col_min = (col_max + col_min) / 2,
                'L' => col_max = (col_max + col_min) / 2,
                _ => continue,
            }
        }
        (row_max - 1) * 8 + (col_max - 1)
    }))
    .collect();
    println!("{:#?}", part1(&input));
    println!("{:#?}", part2(&input));
}

fn part1(input: &Vec<usize>) -> usize {
    *input.iter().last().unwrap()
}

fn part2(input: &Vec<usize>) -> usize {
    for i in 1..input.len() - 1 {
        if input[i] != input[i + 1] - 1 {
            return input[i] + 1;
        }
    }
    0
}
