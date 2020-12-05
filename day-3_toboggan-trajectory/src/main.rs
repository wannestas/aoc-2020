fn main() {
    let input_raw = include_str!("../input.txt");
    println!("{}", part_1_and_2(input_raw, (3, 1)));
    println!("{}", part_2(input_raw));
}

fn part_1_and_2(input: &str, (x_slope, y_slope): (usize, usize)) -> usize {
    let field: Vec<&str> = input
        .lines()
        .collect();
    let (mut x, mut y) = (0, 0);
    let mut tree_counter = 0;
    while y < field.len() {
        if &field[y].chars().nth(x % 31).unwrap() == &'#' {
            tree_counter += 1;
        }
        x += x_slope;
        y += y_slope;
    }
    tree_counter
}

fn part_2(input: &str) -> usize {
    part_1_and_2(input, (1, 1))
        * part_1_and_2(input, (3, 1))
        * part_1_and_2(input, (5, 1))
        * part_1_and_2(input, (7, 1))
        * part_1_and_2(input, (1, 2))
}
