use common::file;

use crate::boats::{parse, solve, GameResult};

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let results = parse(&lines);

    println!("part 1: {}", solve(&results));

    println!(
        "part 2: {}",
        solve(&vec![GameResult {
            time: 48876981,
            distance: 255128811171623,
        }])
    );
}

mod boats {
    use common::parsing::parse_numbers;
    use std::ops::Div;

    pub fn solve(results: &[GameResult]) -> i64 {
        results
            .iter()
            .map(|r| number_of_better_solutions(r.time, r.distance))
            .fold(1, |l, r| l * r)
    }

    pub struct GameResult {
        pub time: i64,
        pub distance: i64,
    }

    pub fn number_of_better_solutions(total_time: i64, achieved_distance: i64) -> i64 {
        let inputs = infer_inputs_from_distance(total_time, achieved_distance);

        let min = f64::floor(inputs[0]) as i64;
        let max = f64::ceil(inputs[1]) as i64;

        /*
           This problem is represented as graph
           > Distance = Speed * (TotalTime - Speed)
            Because Distance=Speed*Time and the Time left to go anywhere is (TotalTime - Speed)

           The speed that yields the max distance is between intersection of this line and the line
           > Distance = achieved_distance
            Because our line:
                > D = S * (t - S)
                ie > D = tS - S^2
            has a negative coefficient of S^2, so is an upside down parabola

           Find intersections and calculate all the ints between them.
           Those are the better solutions
        */
        let i64s_between_min_and_max = (min + 1)..max;

        return i64s_between_min_and_max.into_iter().count() as i64;
    }

    fn infer_inputs_from_distance(total_time: i64, distance: i64) -> [f64; 2] {
        let sqrt_part = f64::sqrt(((total_time * total_time) - 4 * distance) as f64);

        let max = (total_time as f64 + sqrt_part) / 2 as f64;
        let min = (total_time as f64 - sqrt_part) / 2 as f64;

        return [min, max];
    }

    pub fn parse(lines: &Vec<String>) -> Vec<GameResult> {
        let mut line_iter = lines.iter();

        let times = parse_line(line_iter.next().unwrap());
        let distances = parse_line(line_iter.next().unwrap());

        let mut results = Vec::<GameResult>::new();
        for (t, d) in times.iter().zip(distances.iter()) {
            results.push(GameResult {
                time: t.clone(),
                distance: d.clone(),
            })
        }

        return results;
    }

    pub fn parse_line(line: &str) -> Vec<i64> {
        let mut parts = line.split(":");
        let _title = parts.next().unwrap();

        parse_numbers::<i64>(parts.next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::boats::*;
    use common::file;

    #[test]
    fn can_solve_part_1_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let results = parse(&lines);

        assert_eq!(solve(&results), 288);
    }

    #[test]
    fn can_solve_part_1_for_example() {
        let results = vec![
            GameResult {
                time: 7,
                distance: 9,
            },
            GameResult {
                time: 15,
                distance: 40,
            },
            GameResult {
                time: 30,
                distance: 200,
            },
        ];

        assert_eq!(solve(&results), 288);
    }

    #[test]
    fn can_solve_part_2_for_example() {
        let results = vec![GameResult {
            time: 71530,
            distance: 940200,
        }];

        assert_eq!(solve(&results), 71503);
    }

    #[test]
    fn can_find_number_of_better_solutions() {
        assert_eq!(number_of_better_solutions(7, 9), 4);
        assert_eq!(number_of_better_solutions(15, 40), 8);
        assert_eq!(number_of_better_solutions(30, 200), 9);
    }
}
