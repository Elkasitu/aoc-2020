use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;


fn get_parsed_input() -> Vec<String> {
    fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong while reading the input file")
        .split("\n\n")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}


fn main() {
    let groups = get_parsed_input();
    let mut group_occurrences: Vec<HashSet<char>> = Vec::new();
    let mut count_part_2 = 0;
    for group in groups {
        let mut set: HashSet<char> = HashSet::new();
        let mut map: HashMap<char, i32> = HashMap::new();
        let line_count: i32 = group.lines().collect::<Vec<&str>>().len() as i32;
        for line in group.lines() {
            for c in line.chars() {
                set.insert(c);
                *map.entry(c).or_insert(0) += 1;
            }
        }
        for (_, v) in map.iter() {
            if *v == line_count {
                count_part_2 += 1;
            }
        }
        group_occurrences.push(set);
    }
    let mut count = 0;
    for set in group_occurrences {
        count += set.len();
    }
    println!("Santa you fat fuck, the sum of the counts to the form is {}", count);
    println!("Ok so for part 2 the result is {}", count_part_2);
}
