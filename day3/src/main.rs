use std::collections::HashSet;

fn main() {
    let input = "input.txt";
    let mut houses_visited = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut current_coords: (i32, i32) = (0, 0);
    let input = std::fs::read_to_string(input).unwrap().to_lowercase();
    let parsed = input.trim();

    parsed.chars().for_each(|c| {
        match c {
            '>' => {
                current_coords.0 += 1;
            }
            '<' => {
                current_coords.0 -= 1;
            }
            '^' => {
                current_coords.1 += 1;
            }
            'v' => {
                current_coords.1 -= 1;
            }
            _ => (),
        }
        if !visited.contains(&current_coords) {
            visited.insert(current_coords);
            houses_visited += 1;
        }
    });


    println!("{houses_visited}");
}
