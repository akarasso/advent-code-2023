use std::collections::{HashSet};
use regex::{Regex};

pub struct Game {
    pub id: u64,
    pub winning_cards: HashSet<u64>,
    pub player_cards: HashSet<u64>,
}

fn parse_winning_cards(game: &mut Game, raw_winning_cards: &str) {
    let array_of_raw_winning_cards = raw_winning_cards.split(" ");
    for raw_winning_card in array_of_raw_winning_cards {
        if raw_winning_card.trim() == "" {
            continue;
        }
        let card_number = raw_winning_card.parse().unwrap();
        game.winning_cards.insert(card_number);
    }
}

fn parse_player_cards(game: &mut Game, raw_player_cards: &str) {
    let array_of_raw_player_cards: std::str::Split<'_, &str> = raw_player_cards.split(" ");
    for raw_player_card in array_of_raw_player_cards {
        if raw_player_card.trim() == "" {
            continue;
        }
        let card_number = raw_player_card.parse().unwrap();
        game.player_cards.insert(card_number);
    }
}

pub fn parse(line: String) -> Game {
    let re = Regex::new(r"Card\s*(?<game_id>\d+):(?<cards>.*)").unwrap();
    let re_result = re.captures(&line).unwrap();
    let game_id = re_result.name("game_id").unwrap().as_str();
    let cards = re_result.name("cards").unwrap().as_str();
    let parts: Vec<&str> = cards.split("|").into_iter().collect();
    let raw_winning_cards = parts[0];
    let raw_player_cards = parts[1];
    let mut game =  Game{
        id: game_id.parse().unwrap(),
        player_cards: HashSet::new(),
        winning_cards: HashSet::new(),
    };
    parse_winning_cards(&mut game, raw_winning_cards);
    parse_player_cards(&mut game, raw_player_cards);

    return game;
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::parse;

    #[test]
    fn parse_line() {
        let result = parse(String::from("Card   1:  9 32  7 82 10 36 31 12 85 95 |  7 69 23  9 32 22 47 10 95 14 24 71 57 12 31 59 36 68  2 82 38 80 85 21 92"));
        assert_eq!(result.id, 1);
        let expected_winning_card: HashSet<u64> = HashSet::from([9, 32, 7, 82, 10, 36, 31, 12, 85, 95]);
        let expected_player_card: HashSet<u64> = HashSet::from([7, 69, 23, 9, 32, 22, 47, 10, 95, 14, 24, 71, 57, 12, 31, 59, 36, 68, 2, 82, 38, 80, 85, 21, 92]);
        assert!(result.winning_cards.difference(&expected_winning_card).count() == 0);
        assert!(result.player_cards.difference(&expected_player_card).count() == 0);
    }
}
