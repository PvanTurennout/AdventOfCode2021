pub mod aoc_shared{
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
        time::{Instant, Duration},
    };

    pub fn lines_from_resource_file(filename: &str) -> Vec<String> {
        let resource_folder_path = "C:\\Users\\Pieter van Turennout\\Documents\\Projects\\Prive\\AdventOfCode2021\\aoc-2021\\resources\\";
        // let resource_folder_path = "C:\\Users\\piete\\Documents\\Projecten\\Prive\\AdventOfCode2021\\aoc-2021\\resources\\";
        let full_path = resource_folder_path.to_owned() + filename;
        let path = Path::new::<String>(&full_path);

        let file = File::open(<Path as AsRef<Path>>::as_ref(path)).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }

    pub fn time_function_execution(test_subject: fn() -> ()){
        let start = Instant::now();
        test_subject();
        let duration = start.elapsed();
        println!("Function x took {:?} to execute", duration)
    }
}