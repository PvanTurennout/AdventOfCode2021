pub mod day3 {
    use crate::shared::aoc_shared::*;

    pub fn execute_part_1() {
        time_function_execution(part_1);
    }

    pub fn execute_part_2() {
        time_function_execution(part_2);
    }

    fn part_1() {
        let lines = lines_from_resource_file("input_day_3.txt");
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
        println!("Result of part 1 is: {}", result);
    }

    fn find(list: Vec<String>, mut index: usize, oxygen: bool) -> String {
        if list.len() == 1{
            return list[0].clone();
        }

        let mut tb_1 = Vec::new();
        let mut tb_0 = Vec::new();

        for line in list {
            let bits = line.as_bytes();
            if bits[index] == 49 {
                tb_1.push(line);
            } else {
                tb_0.push(line);
            }
        }

        index += 1;

        return if tb_1.len() < tb_0.len() {
            if oxygen {
                find(tb_0, index, oxygen)
            } else {
                find(tb_1, index, oxygen)
            }
        } else {
            if oxygen {
                find(tb_1, index, oxygen)
            } else {
                find(tb_0, index, oxygen)
            }
        }
    }

    fn part_2() {
        let lines = lines_from_resource_file("input_day_3.txt");
        let oxygen_as_string = find(lines, 0, true);
        println!("oxygen string: {}", oxygen_as_string);

        let lines = lines_from_resource_file("input_day_3.txt");
        let co2_as_string = find(lines, 0, false);
        println!("co2 string: {}", co2_as_string);
        println!("Multiply manually because rust is difficult and i am lazy");
    }
}