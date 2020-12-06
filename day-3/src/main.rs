use std::fs;


struct Map {
    content: Vec<Vec<char>>,
    current_pos: (i32, i32),
    y_boundary: usize,
    x_boundary: usize,
}


impl Map {
    pub fn new(content: Vec<Vec<char>>) -> Self {
        let y_boundary = content.len();
        let x_boundary = content[0].len();
        Self {
            content,
            current_pos: (0, 0),
            y_boundary,
            x_boundary,
        }
    }

    pub fn advance(&mut self, slope: (i32, i32)) -> Option<bool> {
        // if x goes out of bounds, the pattern repeats, thus:
        let x = (self.current_pos.0 + slope.0) as usize % self.x_boundary;
        let y = (self.current_pos.1 + slope.1) as usize;
        self.current_pos = (x as i32, y as i32);
        if y >= self.y_boundary {
            None
        } else {
            match self.content[y][x] {
                '.' => Some(false),
                '#' => Some(true),
                _ => panic!("Well that ain't right")
            }
        }
    }
}


fn get_parsed_input() -> Vec<Vec<char>> {
    fs::read_to_string("puzzle_input.txt")
        .expect("Something went wrong while reading the input file")
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect()
}


fn main() {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut slope_results: Vec<i32> = Vec::new();
    let mut tree_count;
    let mut end_reached;
    for slope in slopes {
        let mut map = Map::new(get_parsed_input());
        tree_count = 0;
        end_reached = false;
        while !end_reached {
            match map.advance(slope) {
                Some(true) => tree_count += 1,
                None => end_reached = true,
                _ => continue,
            }
        }
        slope_results.push(tree_count);
    }
    println!("Santa! You hit {} trees on your way down the slope!", slope_results[1]);
    let total_folded = slope_results.iter().fold(1i64, |x, y| x as i64 * *y as i64);
    println!("The number of trees hit per slope multiplied together is {}", total_folded);
}
