use std::fs;
use regex::RegexSet;


fn get_parsed_input() -> Vec<String> {
    fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong whilereading the input file")
        .split("\n\n")      // blank line denotes end of document data
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}


fn validate_identification(set: &RegexSet, input: &str) -> bool {
    let matches: Vec<_> = set.matches(input).into_iter().collect();
    matches == vec![0, 1, 2, 3, 4, 5, 6] || matches == vec![0, 1, 2, 3, 4, 5, 6, 7]
}


fn main() {
    let PART1_RE = RegexSet::new(&[
            r"byr:",
            r"iyr:",
            r"eyr:",
            r"hgt:",
            r"hcl:",
            r"ecl:",
            r"pid:",
            r"cid:",
        ]).unwrap();
    let PART2_RE = RegexSet::new(&[
            r"byr:(19[2-9][0-9]|200[0-2])\b",
            r"iyr:(201[0-9]|2020)\b",
            r"eyr:(202[0-9]|2030)\b",
            r"hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)\b",
            r"hcl:#[0-9a-f]{6}\b",
            r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b",
            r"pid:\d{9}\b",
            r"cid:",
        ]).unwrap();

    let parsed_input = get_parsed_input();
    let mut valid_id_count_part_1 = 0;
    let mut valid_id_count_part_2 = 0;
    for raw_id in parsed_input {
        if validate_identification(&PART1_RE, &raw_id) {
            valid_id_count_part_1 += 1;
        }
        if validate_identification(&PART2_RE, &raw_id) {
            valid_id_count_part_2 += 1;
        }
    }
    println!("Santa, there's a total of {} valid IDs for Part 1", valid_id_count_part_1);
    println!("Santa, there's a total of {} valid IDs for Part 2", valid_id_count_part_2);
}
