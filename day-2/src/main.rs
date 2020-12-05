use std::fs;

struct Policy {
    min: i32,
    max: i32,
    letter: char,
}

struct Password {
    content: String,
    policy: Policy,
}

fn get_parsed_input() -> Vec<String> {
    fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong while reading the input file")
        .split('\n')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn range_to_pair(range: &str) -> (i32, i32) {
    let mut iter = range.split('-')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn str_to_policy(input: &str) -> Policy {
    let mut iter = input.split(' ');
    let range = iter.next().unwrap();
    let letter = iter.next().unwrap();
    let min_max = range_to_pair(range);
    Policy { min: min_max.0, max: min_max.1, letter: letter.chars().next().unwrap() }
}

fn generate_passwords() -> Vec<Password> {
    let input = get_parsed_input();
    let mut output: Vec<Password> = Vec::new();
    for e in input.iter() {
        let mut iter = e.split(':')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        let pol = str_to_policy(iter.next().unwrap());
        let pwd = iter.next().unwrap().to_string();
        output.push(Password { content: pwd, policy: pol });
    }
    output
}

fn is_password_valid_part_1(password: &Password) -> bool {
    let pol = &password.policy;
    let pass = &password.content;
    let matches = pass.matches(pol.letter).count() as i32;
    matches >= pol.min && matches <= pol.max
}

fn is_password_valid_part_2(password: &Password) -> bool {
    let pol = &password.policy;
    let pass = &password.content.as_bytes();
    let n1 = (pol.min - 1) as usize;
    let n2 = (pol.max - 1) as usize;
    (pass[n1] as char == pol.letter && pass[n2] as char != pol.letter) ||
        (pass[n1] as char != pol.letter && pass[n2] as char == pol.letter)
}

fn main() {
    let passwords = generate_passwords();
    let mut valid_count_part_1 = 0;
    let mut valid_count_part_2 = 0;
    for password in passwords.iter() {
        if is_password_valid_part_1(&password) {
            valid_count_part_1 += 1;
        }
        if is_password_valid_part_2(&password) {
            valid_count_part_2 += 1;
        }
    }
    println!("The total amount of valid passwords for part 1 is {}", valid_count_part_1);
    println!("The total amount of valid passwords for part 2 is {}", valid_count_part_2);
}
