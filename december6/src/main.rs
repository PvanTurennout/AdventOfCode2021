use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file() -> Vec<String> {
    let file = File::open("C:\\Users\\piete\\Documents\\Projecten\\Prive\\AdventOfCode2021\\december6\\src\\test.txt")
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

    println!("Result should be: {}", fishes.len()); // 388.419
}

fn part_2() {
    let lines = lines_from_file();
    let numbers = lines[0].split(",");
    let mut fish_count: i128 = 0;


    println!("Result should be: {}", fish_count); // 388419
}

fn main() {
    part_2()
}
