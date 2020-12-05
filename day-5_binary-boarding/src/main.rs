use itertools::sorted;

fn main() {
    let raw_input = include_str!("../input.txt");
    println!("{:#?}", part1(raw_input));
    println!("{:#?}", part2(raw_input));
}

fn part1(input: &str) -> usize {
    //let seat_id: Vec<usize> =
    input
        .split_whitespace()
        .map(|x: &str| {
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
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let input = sorted(input
        .split_whitespace()
        .map(|x: &str| {
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
        .collect::<Vec<usize>>();
    //println!("{:#?}", input);
    for i in 1..input.len()-1 {
        if input[i] != input[i+1] - 1 {
            return input[i]+1;
        }
    }
    0
}
