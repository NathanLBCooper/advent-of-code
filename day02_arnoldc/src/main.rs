use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    thread, time,
};

use common::file;

fn main() {
    let mut console = Command::new("conhost.exe")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let console_stdin = console.stdin.as_mut().unwrap();
    // console_stdin
    //     .write(b"java -jar ArnoldC.jar main.arnoldc \r\n")
    //     .unwrap();

    console_stdin.write(b"java main  \r\n").unwrap();
    thread::sleep(time::Duration::from_millis(1000));

    let inputs = get_arnolds_input("./input.txt");

    for input in inputs {
        thread::sleep(time::Duration::from_millis(50));
        console_stdin
            .write(format!("{} \r\n", input).as_bytes())
            .unwrap();
    }

    thread::sleep(time::Duration::from_millis(1000));
    console_stdin.write(b"exit  \r\n").unwrap();
    console.wait().unwrap();

    let stdout = console.stdout.as_mut().unwrap();
    let out_reader = BufReader::new(stdout);
    for line in out_reader.lines() {
        println!("{}", line.unwrap());
    }

    let stderr = console.stderr.as_mut().unwrap();
    let err_reader = BufReader::new(stderr);
    for line in err_reader.lines() {
        println!("{}", line.unwrap());
    }

    // todo this gave 1830 as the answer, which is wrong
}

fn get_arnolds_input(filename: &str) -> Vec<i32> {
    let file = String::from(filename);
    let lines = file::read_lines(&file).unwrap();

    let games = block_game::deserialize_games(&lines);

    let mut output = Vec::<i32>::new();
    output.push(games.len() as i32);

    for game in games {
        output.push(game.id);
        output.push(game.rounds.len() as i32);

        for round in game.rounds {
            output.push(round.red);
            output.push(round.green);
            output.push(round.blue);
        }
    }

    return output;
}

mod block_game {
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
