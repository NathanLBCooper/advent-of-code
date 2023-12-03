use crate::engine_schematic::Schematic;
use common::file;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let schematic = Schematic::deserialize(&lines);
    let sum = schematic.sum_part_numbers();
    let sum_2 = schematic.sum_part_numbers_2();

    println!("Part one answer is  {}", sum);
    println!("Part two answer is  {}", sum_2);
}

mod engine_schematic {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};

    pub struct Schematic {
        numbers: HashMap<Coordinate, i32>,
        symbols: HashMap<Coordinate, String>,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Coordinate {
        x: i32,
        y: i32,
    }

    impl Schematic {
        pub fn deserialize(lines: &Vec<String>) -> Self {
            let mut numbers = HashMap::<Coordinate, i32>::new();
            let mut symbols = HashMap::<Coordinate, String>::new();

            let num_regex = Regex::new(r"\d+").unwrap();
            let symbol_regex = Regex::new(r#"[^\d.]"#).unwrap();

            for (y, line) in lines.iter().enumerate() {
                for num_match in num_regex.find_iter(line) {
                    let match_text = &line[num_match.start()..num_match.end()];
                    numbers.insert(
                        Coordinate {
                            x: num_match.start() as i32,
                            y: y as i32,
                        },
                        match_text.parse::<i32>().unwrap(),
                    );
                }
                for symbol_match in symbol_regex.find_iter(line) {
                    let match_text = &line[symbol_match.start()..symbol_match.end()];
                    symbols.insert(
                        Coordinate {
                            x: symbol_match.start() as i32,
                            y: y as i32,
                        },
                        match_text.to_string(),
                    );
                }
            }

            return Schematic {
                numbers: numbers,
                symbols: symbols,
            };
        }

        pub fn sum_part_numbers(&self) -> i32 {
            let mut sum = 0;
            for number in &self.numbers {
                if Self::is_part_number(number, &self.symbols) {
                    sum += number.1;
                }
            }

            return sum;
        }

        fn is_part_number(
            number: (&Coordinate, &i32),
            symbols: &HashMap<Coordinate, String>,
        ) -> bool {
            let left_x = number.0.x;
            let right_x = number.0.x + get_number_length(number.1) - 1;
            let y = number.0.y;

            // check above, below, including diagonals
            for iy in [y - 1, y + 1] {
                for ix in left_x - 1..=right_x + 1 {
                    let adjacent = Coordinate { x: ix, y: iy };

                    if symbols.contains_key(&adjacent) {
                        return true;
                    }
                }
            }

            // check left and right
            for ix in [left_x - 1, right_x + 1] {
                let adjacent = Coordinate { x: ix, y: y };

                if symbols.contains_key(&adjacent) {
                    return true;
                }
            }

            return false;
        }

        pub fn sum_part_numbers_2(&self) -> i32 {
            let symbol_shapes: Vec<(Shape, &str)> = self
                .symbols
                .iter()
                .map(|s| (build_hollow_shape_around_point(s.0), s.1 as &str))
                .collect();

            let number_shapes: Vec<(Shape, &i32)> = self
                .numbers
                .iter()
                .map(|s| {
                    (
                        build_shape_on_line(s.0.x, s.0.x + get_number_length(s.1) - 1, s.0.y),
                        s.1,
                    )
                })
                .collect();

            const IS_PART_2: bool = true;
            let mut total_sum = 0;
            for symbol_shape in symbol_shapes {
                if IS_PART_2 && symbol_shape.1 != "*" {
                    continue;
                }
                let mut symbol_sum = 0;
                let mut how_many_matches = 0;
                'number: for number_shape in number_shapes.iter() {
                    for coord in symbol_shape.0.area.iter() {
                        if number_shape.0.area.contains(coord) {
                            if IS_PART_2 && symbol_sum != 0 {
                                symbol_sum *= number_shape.1;
                                how_many_matches += 1;
                                continue 'number;
                            } else {
                                symbol_sum += number_shape.1;
                                how_many_matches += 1;
                                continue 'number;
                            }
                        }
                    }
                }

                // todo shit code
                if how_many_matches < 2 {
                    symbol_sum = 0;
                }
                total_sum += symbol_sum;
            }

            return total_sum;
        }
    }

    struct Shape {
        area: HashSet<Coordinate>,
    }

    fn build_hollow_shape_around_point(point: &Coordinate) -> Shape {
        let mut area = HashSet::<Coordinate>::new();
        for iy in [point.y - 1, point.y + 1] {
            for ix in point.x - 1..=point.x + 1 {
                let adjacent = Coordinate { x: ix, y: iy };
                area.insert(adjacent);
            }
        }

        for ix in [point.x - 1, point.x + 1] {
            let adjacent = Coordinate { x: ix, y: point.y };
            area.insert(adjacent);
        }

        return Shape { area };
    }

    fn build_shape_on_line(left_x: i32, right_x: i32, y: i32) -> Shape {
        let mut area = HashSet::<Coordinate>::new();
        for ix in left_x..=right_x {
            let adjacent = Coordinate { x: ix, y: y };
            area.insert(adjacent);
        }

        return Shape { area };
    }

    fn get_number_length(n: &i32) -> i32 {
        return n.to_string().chars().count() as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::engine_schematic::*;
    use common::file;

    #[test]
    fn can_find_sum_of_part_numbers_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let schematic = Schematic::deserialize(&lines);
        let sum = schematic.sum_part_numbers();

        assert_eq!(sum, 4361);
    }

    #[test]
    fn can_find_sum_of_part_numbers_for_example_file_part_2() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let schematic = Schematic::deserialize(&lines);
        let sum = schematic.sum_part_numbers_2();

        //assert_eq!(sum, 4361);
        assert_eq!(sum, 467835);
    }
}
