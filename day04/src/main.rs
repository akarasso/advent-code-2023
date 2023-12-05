mod game_parser;
mod game_analyzer;
use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

use crate::game_analyzer::GameState;

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

fn main() {
    // Q1
    // let mut total_of_point: i32 = 0;
    // let games_input = read_input();
    // for game_input in games_input {
    //     let parsed_game = game_parser::parse(game_input);
    //     let game_result = game_analyzer::game_analyzer(&parsed_game);
    //     total_of_point += game_result.point as i32;
    // }
    // println!("{}", total_of_point);

    // Q2 => Failed du to implementation of hashset..
    let mut total_of_point: i32 = 0;
    let mut player_cards: HashSet<u64> = HashSet::new();
    let games_input = read_input();
    for game_input in games_input {
        let mut parsed_game = game_parser::parse(game_input);
        for card in &player_cards {
            parsed_game.player_cards.insert(*card);
        }
        let game_result = game_analyzer::game_analyzer(&parsed_game);
        if game_result.state == GameState::Loose {
            player_cards.clear();
        } else {
            for card in game_result.player_winning_card {
                if player_cards.iter().count() < 30 {
                    player_cards.insert(card);
                }
            }
        }
        total_of_point += player_cards.iter().count() as i32;
    }
    println!("{}", total_of_point);
}
