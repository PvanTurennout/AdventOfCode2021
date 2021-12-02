use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

struct Position{
    horizontal: i16,
    depth: i32,
    aim: i16
}

impl Position {
    fn new (horizontal: i16, depth: i32, aim: i16) -> Position{
        return Position{ horizontal, depth, aim };
    }

    fn init () -> Position{
        return Position::new(0, 0, 0);
    }

    fn execute_command_part_1(&mut self, command: Command){
        if command.direction == "forward" {
            self.horizontal += command.amount;
            return;
        }

        if command.direction == "up" {
            self.depth -= command.amount as i32;
            return;
        }

        if command.direction == "down" {
            self.depth += command.amount as i32;
            return;
        }
    }

    fn execute_command_part_2(&mut self, command: Command){
        if command.direction == "forward" {
            self.horizontal += command.amount;
            self.depth += command.amount as i32 * self.aim as i32;
            return;
        }

        if command.direction == "up" {
            self.aim -= command.amount;
            return;
        }

        if command.direction == "down" {
            self.aim += command.amount;
            return;
        }
    }

    fn get_answer(&self) -> i32{
        return self.horizontal as i32 * self.depth as i32;
    }
}

struct Command{
    direction: String,
    amount: i16
}

impl Command {
    fn new(direction: &str, amount: i16) -> Command{
        return Command{ direction: String::from(direction), amount };
    }
}

fn lines_from_file() -> Vec<String> {
    let file = File::open("C:\\Users\\Pieter van Turennout\\Documents\\Projects\\Prive\\AdventOfCode2021\\december2\\src\\input.txt")
        .expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn commands_from_lines(lines: Vec<String>) -> Vec<Command> {
    let mut commands = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        let dir = split.get(0).unwrap();
        let am = split.get(1).unwrap().parse::<i16>().unwrap();
        commands.push(Command::new(dir, am));
    }

    return commands
}

fn part_1(commands: Vec<Command>) -> i32 {
    let mut position = Position::init();

    for command in commands {
        position.execute_command_part_1(command);
    }

    return position.get_answer()
}

fn part_2(commands: Vec<Command>) -> i32 {
    let mut position = Position::init();

    for command in commands {
        position.execute_command_part_2(command);
    }

    return position.get_answer()
}

fn main() {
    let lines = lines_from_file();
    let commands =  commands_from_lines(lines);

    // let result = part_1(commands);
    let result = part_2(commands);

    println!("the amount is {}", result);
}
