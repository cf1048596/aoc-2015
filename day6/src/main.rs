use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "input.txt";
    let mut grid: Vec<Vec<u8>> = vec![vec![0; 1000]; 1000];
    let file = std::fs::read_to_string(input)?;
    for line in file.lines() {
        println!("{line}");

        let words: Vec<String> = line.split(' ').map(String::from).collect();

        let numbers: Vec<String> = words
            .iter()
            .filter(|x| x.chars().next().is_some_and(|x| x.is_ascii_digit()))
            .cloned()
            .collect();

        if let Some(first_words) = words.windows(2).next() {
            let first_word = first_words.first().unwrap();
            let second_word = first_words.get(1).unwrap();
            let [first_set, second_set]: [Vec<i32>; 2] = numbers
                .iter()
                .take(2)
                .map(|line| line.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            match (first_word.as_str(), second_word.as_str()) {
                ("turn", "off") => {
                    for row in *first_set.first().unwrap()..=*second_set.first().unwrap() {
                        for col in *first_set.get(1).unwrap()..=*second_set.get(1).unwrap() {
                            let cell = grid[row as usize][col as usize];
                            if cell >= 1 {
                                grid[row as usize][col as usize] -= 1;
                            }
                        }
                    }
                }
                ("turn", "on") => {
                    for row in *first_set.first().unwrap()..=*second_set.first().unwrap() {
                        for col in *first_set.get(1).unwrap()..=*second_set.get(1).unwrap() {
                            grid[row as usize][col as usize] += 1;
                        }
                    }
                }
                ("toggle", _) => {
                    for row in *first_set.first().unwrap()..=*second_set.first().unwrap() {
                        for col in *first_set.get(1).unwrap()..=*second_set.get(1).unwrap() {
                            grid[row as usize][col as usize] += 2;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let lit_lights: u64 = grid.iter().flatten().map(|x| u64::from(*x)).sum();
    println!("lit lights: {lit_lights}");
    Ok(())
}
