use std::collections::HashSet;

fn main() {
    let input = "input.txt";
    let mut houses_visited = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut current_coords: (i32, i32) = (0, 0);
    let mut mirrored_current_coords: (i32, i32) = (0, 0);
    let input = std::fs::read_to_string(input).unwrap().to_lowercase();
    let parsed = input.trim();

    let mut which_santa: bool = false;


    parsed.chars().for_each(|c| {
        match (c, which_santa) {
            ('>', false) => {
                current_coords.0 += 1;
            }
            ('<', false) => {
                current_coords.0 -= 1;
            }
            ('^', false) => {
                current_coords.1 += 1;
            }
            ('v', false) => {
                current_coords.1 -= 1;
            }
            ('>', true) => {
                mirrored_current_coords.0 += 1;
            }
            ('<', true) => {
                mirrored_current_coords.0 -= 1;
            }
            ('^', true) => {
                mirrored_current_coords.1 += 1;
            }
            ('v', true) => {
                mirrored_current_coords.1 -= 1;
            }
            _ => (),
        }
        which_santa = !which_santa;
        if !visited.contains(&current_coords) {
            visited.insert(current_coords);
            houses_visited += 1;
        } else if !visited.contains(&mirrored_current_coords) {
            visited.insert(mirrored_current_coords);
            houses_visited += 1;
        }
    });

    println!("{houses_visited}");
}
