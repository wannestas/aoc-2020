use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let numbers: Vec<usize> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    println!("The combination is: {:?}", two_complement(&numbers, 2020));
    println!("The combination is: {:?}", three_complement(&numbers));
    Ok(())
}

fn two_complement(numbers: &Vec<usize>, complementary_to: usize) -> Vec<usize> {
    for i in numbers {
        if i > &complementary_to {
            continue;
        }
        let complement: usize = complementary_to - i;
        if numbers.iter().any(|&x| x == complement) {
            let complements_vec: Vec<usize> = vec![complement, complementary_to - complement];
            return complements_vec;
        }
    }
    vec![0, 0]
}

fn three_complement(numbers: &Vec<usize>) -> Vec<usize> {
    for i in numbers {
        let complement = i;
        let two_complements_vec = two_complement(&numbers, 2020 - complement);
        if numbers.iter().any(|&x| x == two_complements_vec[0]) {
            return vec![two_complements_vec[0], two_complements_vec[1], *complement];
        }
    }
    vec![0, 0, 0]
}
