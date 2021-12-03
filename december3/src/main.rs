use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file() -> Vec<String> {
    let file = File::open("C:\\Users\\Pieter van Turennout\\Documents\\Projects\\Prive\\AdventOfCode2021\\december3\\src\\input.txt")
        .expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1() -> u32{
    let lines = lines_from_file();
    let threshold = (lines.len() / 2) as u16;

    let mut count: [u16; 12] = [0; 12];

    for line in lines {
        println!("working on line: {}", line);

        let bits = line.as_bytes();
        for i in 0..bits.len() {
            if bits[i] == 49 {
                let c = count[i] as u32 + 1;
                count[i] = c as u16;
            }
        }
    }

    println!("piecing together gamma");
    let mut gamma = 0b0000_0000_0000_0000_u16;

    for i in 0..count.len() {
        if count[i] > threshold{
            let insert = 0b0000_1000_0000_0000_u16 >> i;
            gamma |= insert;
        }
    }

    let epsilon = !gamma & 0b0000_1111_1111_1111;

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);


    let result = gamma as u32 * epsilon as u32;
    return result;
}

fn part_2() -> u32 {
    let lines = lines_from_file();

    let mut oxygen_list = Vec::new();
    let mut co2_list = Vec::new();



    while oxygen_list.len() > 1{

    }

    while co2_list.len() > 1{

    }

    return 0;
}

fn main() {
    // let result = part_1();
    let result = part_2();

    println!("Result should be: {}", result);
}