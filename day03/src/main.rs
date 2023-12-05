mod engine_parser;
use std::{fs::File, io::{BufReader, BufRead}, collections::{hash_map, HashMap}};

fn create_input_reader() -> BufReader<File> {
    let data_result = File::open("input2");
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };
    return BufReader::new(data_file);
}

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = vec![];
    let reader: BufReader<File> = create_input_reader();
    for line_result in reader.lines() {
        let line_read = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem opening the data file: {:?}", error),
        };
        input.push(line_read)
    }
    return input;
}

struct AnswerQ2 {
    multiplyed_times: u32,
    value: u64,
}

fn main() {
    let mut sum2 = 0;
    let mut sum = 0;
    let engine_map = read_input();
    let engine = engine_parser::parse(engine_map);
    // Q1
    for engine_part in engine.parts.iter() {
        sum += engine_part.value
    }
    println!("Sum: {}", sum);
    // Q2
    let mut hmap: HashMap<String, AnswerQ2> = HashMap::new();
    for engine_part in engine.parts.iter() {
        if engine_part.key.key != '*' {
            continue;
        }
        println!("{} {}", engine_part.key.index_line + 1, engine_part.key.index_char + 1);
        let index_key = engine_part.key.index_line.to_string() + "_" + &engine_part.key.index_char.to_string();
        let current = hmap.get(&index_key);
        if current.is_none() {
            hmap.insert(index_key, AnswerQ2{
                multiplyed_times: 1,
                value: engine_part.value,
            });
        } else {
            let current_value = current.unwrap();
            hmap.insert(index_key.clone(), AnswerQ2{
                multiplyed_times: current_value.multiplyed_times + 1,
                value: current_value.value * engine_part.value,
            });
        }
    }
    for (_key, response) in hmap {
        if response.multiplyed_times > 1 {
            sum2 += response.value;
        }
    }
    println!("Sum2: {}", sum2);
}
