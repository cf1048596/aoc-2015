fn main() {
    let input = "input.txt";
    let input_str: String = std::fs::read_to_string(input).unwrap();

    let mut stack: Vec<char> = Vec::new();

    let mut sum: i32 = 0;
    let mut first_basement_pos = 0;
    let mut entered_basement = false;
    for (i, c) in input_str.chars().enumerate() {
        match c {
            '(' => {
                stack.push(c);
                sum += 1;
            }
            ')' => {
                stack.pop();
                sum -= 1;
            }
            _ => (),
        }

        if !entered_basement && sum == -1 {
            first_basement_pos = i + 1;
            entered_basement = true;
        }
    }

    println!("{sum}, {first_basement_pos}");
}
