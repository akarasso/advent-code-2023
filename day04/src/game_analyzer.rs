use std::collections::HashSet;

use crate::game_parser::Game;

#[derive(PartialEq)]
pub enum GameState {
    Win,
    Loose,
}


pub struct GameResult {
    pub state: GameState,
    pub point: u64,
    pub player_winning_card: HashSet<u64>
}

pub fn game_analyzer(game: &Game) -> GameResult {
    let difference = game.winning_cards.intersection(&game.player_cards);
    let winning_cards = difference.clone().count();

    let mut player_winning_card: HashSet<u64> = HashSet::new();
    difference.for_each(|value| { player_winning_card.insert(*value); });

    if winning_cards == 0 {
        return GameResult{
            state: GameState::Loose,
            point: 0,
            player_winning_card: player_winning_card,
        };
    }
    return GameResult{
        state: GameState::Win,
        point: 1 << (winning_cards - 1),
        player_winning_card: player_winning_card,
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game_parser::Game;

    use super::game_analyzer;

    #[test]
    fn parse_line() {
        let game = Game{
            id: 1,
            winning_cards: HashSet::from([41, 48, 83, 86, 17]),
            player_cards: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };
        let result = game_analyzer(&game);
        assert_eq!(result.point, 8);
        assert_eq!(result.player_winning_card, HashSet::from_iter([48, 83, 86, 17]));
    }
}
