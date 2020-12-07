use std::fs;


fn get_parsed_input() -> Vec<String> {
    fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong while reading the input file")
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}


fn get_seat_information(code: &str) -> (i32, i32, i32) {
    let mut rows: Vec<i32> = (0..128).collect();
    let mut cols: Vec<i32> = (0..8).collect();
    for c in code.chars() {
        let half_rows: usize = (rows.len() as f64 / 2.0).floor() as usize;
        let half_cols: usize = (cols.len() as f64 / 2.0).floor() as usize;
        match c {
            'F' => rows = rows[0..half_rows].to_vec(),
            'B' => rows = rows[half_rows..].to_vec(),
            'L' => cols = cols[0..half_cols].to_vec(),
            'R' => cols = cols[half_cols..].to_vec(),
            _   => (),
        }
    }
    (rows[0], cols[0], rows[0] * 8 + cols[0])
}


fn get_santa_seat_id(seat_ids: &Vec<i32>) -> i32 {
    let mut all_seats: Vec<i32> = Vec::new();
    for i in 0..128 {
        for j in 0..8 {
            all_seats.push(i * 8 + j);
        }
    }
    for seat in all_seats.iter() {
        if !seat_ids.contains(seat) &&
            seat_ids.contains(&(seat - 1)) &&
            seat_ids.contains(&(seat + 1)) {
            return *seat;
        }
    }
    0

}


fn main() {
    let input = get_parsed_input();
    let mut seat_ids: Vec<i32> = Vec::new();
    for s in input {
        seat_ids.push(get_seat_information(&s).2);
    }
    println!("Santa, {} is the max seat ID found in the system!", &seat_ids.iter().max().unwrap());
    let santa_seat_id = get_santa_seat_id(&seat_ids);
    println!("Santa, your seat ID is {}", santa_seat_id);
}
