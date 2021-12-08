pub mod day1 {
    use crate::shared::aoc_shared::*;

    pub fn execute_part_2(){
        time_function_execution(part_2);
    }

    fn part_2() {
        let lines = lines_from_resource_file("input_day_1.txt");
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
}