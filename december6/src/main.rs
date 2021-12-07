use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file() -> Vec<String> {
    let file = File::open("C:\\Users\\Pieter van Turennout\\Documents\\Projects\\Prive\\AdventOfCode2021\\december6\\src\\input.txt")
        .expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1() {
    let lines = lines_from_file();
    let numbers = lines[0].split(",");
    let mut fishes: Vec<i8> = Vec::new();

    for line in numbers {
        fishes.push(line.parse::<i8>().unwrap());
    }

    let mut day = 0;

    while day != 80 {
        let mut new_count = 0;
        for i in 0..fishes.len() {
            let c = fishes[i] - 1;
            if c == -1 {
                new_count += 1;
                fishes[i] = 6;
                continue;
            }
            fishes[i] = c;
        }

        while new_count != 0 {
            fishes.push(8);
            new_count -= 1;
        }

        day += 1;
    }

    println!("Result is: {}", fishes.len()); // 388.419
}

fn part_2() {
    let mut days: [u64; 256] = [0; 256];
    let lines = lines_from_file();
    let numbers = lines[0].split(",");
    let input_size = numbers.clone().count() as u128;

    for number in numbers {
        let index = number.parse::<i8>().unwrap() as usize;
        days[index] += 1;
    }

    for i in 0..(days.len() - 7) {
        let current = days[i];
        days[i + 7] += current;

        if (i + 9) < days.len() {
            days[i + 9] += current;
        }
    }

    let mut fish_count: u128 = 0;

    for day in days {
        fish_count += day as u128;
    }

    fish_count += input_size;

    println!("Result is: {}", fish_count); // 1740449478328
}

fn main() {
    part_1();
    part_2();
}
