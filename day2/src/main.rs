use std::{fs::File, io::Read};

fn main() {
    let input = "input.txt";
    let mut sum = 0;
    //2*l*w + 2*w*h + 2*h*l
    let input_lines = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    input_lines.iter().for_each(|x| {
        let numbers: Vec<&str> = x.trim().split("x").collect::<Vec<_>>();
        let num_numbers = numbers.iter().len();
        println!("{num_numbers}");
        let length = numbers.first().unwrap().parse::<i32>().unwrap();
        let width = numbers.get(1).unwrap().parse::<i32>().unwrap();
        let height = numbers.get(2).unwrap().parse::<i32>().unwrap();


        let area_1 = 2*length*width;
        let area_2 = 2*width*height;
        let area_3 = 2*height*length;

let min_val = (area_1 / 2).min(area_2 / 2).min(area_3 / 2);
        sum += area_1+area_2+area_3+min_val;
    });
    println!("{sum}");
}
