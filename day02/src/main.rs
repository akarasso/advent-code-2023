mod game_parser;
use std::{fs::File, io::{BufReader, BufRead}};

fn create_input_reader() -> BufReader<File> {
    let data_result = File::open("input");
    // Reading a file returns a Result enum
    // Result can be a file or an error
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };
    return BufReader::new(data_file);
}

/**
 * For day02: first question
 */
fn is_valid_game(game: &game_parser::Game) -> bool {
    return game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14
}

/**
 * For day02: second question
 */
fn calculate_power_of_game(game: &game_parser::Game) -> u64 {
    return game.max_red * game.max_green * game.max_blue
}

fn main() {
    let mut power_of_all_games: u64 = 0;
    let mut answer: u64 = 0;
    let reader: BufReader<File> = create_input_reader();
    for line_result in reader.lines() {
        let line_read = match line_result {
            Ok(line) => line,
            Err(error) => panic!("Problem opening the data file: {:?}", error),
        };
        let game: game_parser::Game = game_parser::parse(line_read);
        if is_valid_game(&game) {
            answer += game.id;
        }
        power_of_all_games += calculate_power_of_game(&game);
    }
    println!("power_of_all_games: {}", power_of_all_games);
    println!("Answer: {}", answer)
}
