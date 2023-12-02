fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let games = block_game::deserialize_games(&lines);

    let bag = block_game::Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    println!("{}", block_game::sum_of_ids_of_possible_games(&bag, &games));

    println!("{}", block_game::sum_of_minimal_bag_powers(&games));
}

mod block_game {
    pub fn sum_of_ids_of_possible_games(bag: &Bag, games: &Vec<Game>) -> i32 {
        return games
            .iter()
            .filter(|g| bag.is_game_possible(g))
            .map(|g| g.id)
            .sum();
    }

    pub fn sum_of_minimal_bag_powers(games: &Vec<Game>) -> i32 {
        return games
            .iter()
            .map(|g| Bag::create_minimal_bag(g))
            .map(|b| b.get_power())
            .sum();
    }

    pub struct Game {
        pub id: i32,
        pub rounds: Vec<Round>,
    }

    pub struct Round {
        pub red: i32,
        pub green: i32,
        pub blue: i32,
    }

    pub struct Bag {
        pub red: i32,
        pub green: i32,
        pub blue: i32,
    }

    impl Bag {
        pub fn is_round_possibe(&self, round: &Round) -> bool {
            return self.red >= round.red && self.green >= round.green && self.blue >= round.blue;
        }

        pub fn is_game_possible(&self, game: &Game) -> bool {
            return game
                .rounds
                .iter()
                .map(|r| self.is_round_possibe(r))
                .all(|p| p);
        }

        pub fn create_minimal_bag(games: &Game) -> Self {
            return Bag {
                red: games.rounds.iter().map(|r| r.red).max().unwrap(),
                green: games.rounds.iter().map(|r| r.green).max().unwrap(),
                blue: games.rounds.iter().map(|r| r.blue).max().unwrap(),
            };
        }

        pub fn get_power(&self) -> i32 {
            return self.red * self.blue * self.green;
        }
    }

    pub fn deserialize_games(lines: &Vec<String>) -> Vec<Game> {
        let games: Vec<Game> = lines.iter().map(|l| deserialize_game_line(l)).collect();

        return games;
    }

    pub fn deserialize_game_line(line: &str) -> Game {
        let parts: Vec<&str> = line.split(":").collect();
        let mut parts_iter = parts.iter();

        let id_part = parts_iter.next().unwrap();
        let id = id_part
            .split_whitespace()
            .into_iter()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let round_parts: Vec<&str> = parts_iter.next().unwrap().split(";").collect();

        let rounds: Vec<Round> = round_parts.iter().map(|p| deserialize_round(p)).collect();

        return Game {
            id: id,
            rounds: rounds,
        };
    }

    fn deserialize_round(text: &str) -> Round {
        let parts: Vec<&str> = text.split(",").collect();

        let (mut red, mut green, mut blue) = (0, 0, 0);
        for part in parts.iter() {
            let words: Vec<&str> = part.split_whitespace().collect();
            let mut words_iter = words.iter();

            let number = words_iter.next().unwrap().parse::<i32>().unwrap();
            let second_word = words_iter.next().unwrap();

            match second_word {
                &"red" => red += number,
                &"green" => green += number,
                &"blue" => blue += number,
                _ => panic!(),
            }
        }

        return Round { red, green, blue };
    }
}

#[cfg(test)]
mod tests {
    use crate::{block_game, file};

    #[test]
    fn can_find_sum_of_possible_game_ids_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let bag = block_game::Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        let games = block_game::deserialize_games(&lines);

        let sum = block_game::sum_of_ids_of_possible_games(&bag, &games);

        assert_eq!(sum, 8);
    }

    #[test]
    fn can_find_sum_of_minimal_bag_powers_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();
        let games = block_game::deserialize_games(&lines);

        let sum = block_game::sum_of_minimal_bag_powers(&games);

        assert_eq!(sum, 2286);
    }

    #[test]
    fn can_deserialize_game() {
        let line = "Game 1: 1 green, 6 red, 4 blue; 2 blue, 6 green, 7 red; 3 red, 4 blue, 6 green; 3 green; 3 blue, 2 green, 1 red";

        let game = block_game::deserialize_game_line(&line);

        assert_eq!(game.id, 1);
        assert_eq!(game.rounds.iter().count(), 5);

        let first_round = game.rounds.iter().nth(0).unwrap();
        assert_eq!(first_round.green, 1);
        assert_eq!(first_round.red, 6);
        assert_eq!(first_round.blue, 4);

        let round_without_all_colors = game.rounds.iter().nth(3).unwrap();
        assert_eq!(round_without_all_colors.green, 3);
        assert_eq!(round_without_all_colors.red, 0);
        assert_eq!(round_without_all_colors.blue, 0);
    }

    #[test]
    fn can_deserialize_file() {
        let example_file = String::from("./example.txt");

        let lines = file::read_lines(&example_file);
        let games = block_game::deserialize_games(&lines.unwrap());

        assert_eq!(games.iter().count(), 5);

        let game = games.iter().nth(1).unwrap();
        assert_eq!(game.id, 2);
        assert_eq!(game.rounds.iter().count(), 3);

        let round = game.rounds.iter().nth(0).unwrap();
        assert_eq!(round.blue, 1);
        assert_eq!(round.green, 2);
        assert_eq!(round.red, 0);
    }
}

// todo repetative
mod file {
    use crate::core::Error;
    use crate::core::Result;
    use std::fs::read_to_string;

    pub fn read_lines(filename: &str) -> Result<Vec<String>> {
        return match read_to_string(filename) {
            Ok(s) => return Ok(s.lines().map(String::from).collect()),
            Err(_) => Err(Error::new("can't read file")),
        };
    }
}

// todo repetative
mod core {
    use std::{fmt::Debug, result};

    pub type Result<T> = result::Result<T, Error>;

    pub struct Error {
        reason: String,
    }

    impl Error {
        pub fn new(reason: &str) -> Self {
            return Self {
                reason: String::from(reason),
            };
        }

        pub fn get_reason(&self) -> &String {
            return &self.reason;
        }
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Error")
                .field("reason", &self.reason)
                .finish()
        }
    }
}
