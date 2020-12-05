use std::fs;


fn main() {
    let raw_input = fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong while reading the input file");
    let numbas: Vec<i32> =
        raw_input.split('\n')
                 .map(|s| s.trim())
                 .filter(|s| !s.is_empty())
                 .map(|s| s.parse().unwrap())
                 .collect();

    let mut result_part_1: i32 = 0;
    let mut result_part_2: i32 = 0;
    for (x, &i) in numbas.iter().enumerate() {
        for (y, &j) in numbas.iter().enumerate() {
            if x != y {
                if i + j == 2020 {
                    result_part_1 = i * j;
                } else {
                    for (z, &k) in numbas.iter().enumerate() {
                        if y != z && i + j + k == 2020 {
                            result_part_2 = i * j * k;
                        }
                    }
                }
            }
        }
    }
    println!("The result for part 1 is {} and for part 2 is {}", result_part_1, result_part_2);
}
