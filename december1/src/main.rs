use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {

    let lines = lines_from_file("C:\\Users\\Pieter van Turennout\\Documents\\Projects\\Prive\\AdventOfCode2021\\december1\\src\\input.txt");
    let mut values = Vec::new();
    let mut grouped_values = Vec::new();

    for line in lines {
        values.push(line.parse::<i32>().unwrap());
    }

    for j in 0..values.len() {
        // exclude last 2 entries
        if j == values.len() - 2 {
            break;
        }

        let value = values.get(j).unwrap() + values.get(j + 1).unwrap() + values.get(j + 2).unwrap();
        grouped_values.push(value);
    }

    let mut count = 0;
    for i in 1..grouped_values.len() {
        if grouped_values.get(i) > grouped_values.get(i - 1) {
            count += 1;
        }
    }

    println!("Amount of increases: {}", count)
}
