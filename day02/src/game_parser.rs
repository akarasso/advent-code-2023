use regex::Regex;

pub struct Game {
    pub id: u64,
    pub max_red: u64,
    pub max_green: u64,
    pub max_blue: u64,
}

fn parse_games(total_of_game: &mut Game, games_line: &str) {
    let game_separator = "; ";
    let games = games_line.trim().split(game_separator);
    for game in games.into_iter() {
        let array_of_value_and_dice_color = game.trim().split(", ").into_iter();
        for value_and_dice_color_string in array_of_value_and_dice_color {
            let value_and_dice_color: Vec<&str> = value_and_dice_color_string.split(" ").collect();
            let value: u64 = value_and_dice_color[0].parse().unwrap();
            let dice_color = value_and_dice_color[1];
            if dice_color == "red" {
                if value > total_of_game.max_red {
                    total_of_game.max_red = value;
                }
            } else if dice_color == "green" {
                if value > total_of_game.max_green {
                    total_of_game.max_green = value;
                }
            } else if dice_color == "blue" {
                if value > total_of_game.max_blue {
                    total_of_game.max_blue = value;
                }
            } else {
                panic!("Unknow color value {}", dice_color)
            }
        }
    }
}

pub fn parse(line: String) -> Game {
    let re = Regex::new(r"Game (?<game_id>\d+): (?<games>.*)").unwrap();
    let re_result: regex::Captures<'_> = re.captures(&line).unwrap();
    let game_id = re_result.name("game_id").unwrap().as_str();
    let games: &str = re_result.name("games").unwrap().as_str();

    let mut total_of_game = Game{
        id: game_id.parse().unwrap(),
        max_red: 0,
        max_green: 0,
        max_blue: 0,
    };
    parse_games(&mut total_of_game, games);

    return total_of_game;
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parse_line() {
        let result = parse(String::from("Game 3: 8 green, 1 blue, 7 red; 12 red, 6 blue, 9 green; 2 blue, 1 red, 10 green; 9 green, 4 red; 2 red, 1 blue, 8 green"));
        assert_eq!(result.id, 3);
        assert_eq!(result.max_green, 10);
        assert_eq!(result.max_blue, 6);
        assert_eq!(result.max_red, 12);
    }
}
