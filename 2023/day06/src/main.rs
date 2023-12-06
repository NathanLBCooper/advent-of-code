use common::file;

use crate::boats::{number_of_better_solutions, parse, part_1, GameResult};

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let results = parse(&lines);

    println!("part 1: {}", part_1(&results));

    println!(
        "part 2: {}",
        number_of_better_solutions(&GameResult {
            time: 48876981,
            distance: 255128811171623,
        })
    );
}

mod boats {
    use common::parsing::parse_numbers;

    pub fn part_1(results: &[GameResult]) -> i64 {
        results
            .iter()
            .map(|r| number_of_better_solutions(&r))
            .fold(1, |l, r| l * r)
    }

    pub struct GameResult {
        pub time: i64,
        pub distance: i64,
    }

    pub fn number_of_better_solutions(result: &GameResult) -> i64 {
        /*
           The problem is represented as a line on a graph
                "Distance = Speed * (Time - Speed)"
           This is a upside down parabola (ie a hill, not a valley)
           A potential solution is also a line: "Distance = CONST_VALUE"
           The line is either intersects with the first line
            0 times (ie impossible), 1 time (ie optimal) or 2 times (ie sub-optimal)
           In the case of 2 times, answers with higher distances are found between those points.
        */

        let inputs = infer_inputs_from_distance(result);

        let int_min_or_below = f64::floor(inputs[0]) as i64;
        let int_max_or_above = f64::ceil(inputs[1]) as i64;
        let better_integer_solutions = (int_min_or_below + 1)..int_max_or_above;

        better_integer_solutions.into_iter().count() as i64
    }

    fn infer_inputs_from_distance(result: &GameResult) -> [f64; 2] {
        /*
           To find intersection between
            "distance = CONST_VALUE" and "Distance = Speed * (Time - Speed)"
           use quadratic formula
            x = (-b +or- sqrt(b^2 -4ac) ) / 2a
           When x is speed and "0 = x^2 - time*x + distance"
            Then a=-1, b=-time, c=distance
        */
        const A: i64 = 1;
        let b = -result.time;
        let c = result.distance;

        let sqrt_part = f64::sqrt(((b * b) - 4 * A * c) as f64);

        let max_solution = (-b as f64 + sqrt_part) / (2 * A) as f64;
        let min_solution = (-b as f64 - sqrt_part) / (2 * A) as f64;

        return [min_solution, max_solution];
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

    fn parse_line(line: &str) -> Vec<i64> {
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

        assert_eq!(part_1(&results), 288);
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

        assert_eq!(part_1(&results), 288);
    }

    #[test]
    fn can_solve_part_2_for_example() {
        let results = vec![GameResult {
            time: 71530,
            distance: 940200,
        }];

        assert_eq!(part_1(&results), 71503);
    }

    #[test]
    fn can_find_number_of_better_solutions() {
        assert_eq!(
            number_of_better_solutions(&GameResult {
                time: 7,
                distance: 9
            }),
            4
        );
        assert_eq!(
            number_of_better_solutions(&GameResult {
                time: 15,
                distance: 40
            }),
            8
        );
        assert_eq!(
            number_of_better_solutions(&GameResult {
                time: 30,
                distance: 200
            }),
            9
        );
    }
}
