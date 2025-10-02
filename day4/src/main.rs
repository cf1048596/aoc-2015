fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    let _ = stdin.read_line(&mut input).unwrap();
    let trimmed = input.trim();
    let mut i = 0;
    'running: loop {
        let to_hash = format!("{trimmed}{i}");
        let hash = md5::compute(&to_hash);
        let output = format!("{hash:?}");

        let first_five: String = output.chars().take(5).collect();

        if first_five.as_str() == "00000" {
            println!("{hash:?}");
            println!("{to_hash}");
            break 'running;

        }
        i += 1;
    }
}
