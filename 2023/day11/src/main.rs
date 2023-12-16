use crate::astronomy::{image::Image, sum_distances_between_galaxy_pairs};
use common::file;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();
    let v: Vec<&str> = lines.iter().map(|x| x.as_ref()).collect();

    let image = Image::new(&v);

    println!("part 1: {}", sum_distances_between_galaxy_pairs(&image, 1));
    println!(
        "part 2: {}",
        sum_distances_between_galaxy_pairs(&image, 1_000_000)
    );
}

mod astronomy {
    use crate::geometry::calculate_path_distance;

    use self::image::Image;

    pub fn sum_distances_between_galaxy_pairs(image: &Image, expansion_factor: i32) -> i64 {
        let galaxies = image.galaxies(expansion_factor);

        let pairs = find_pairs(&galaxies);

        let distances = pairs.iter().map(|p| calculate_path_distance(p.0, p.1));

        distances.sum()
    }

    pub fn find_pairs<T>(collection: &[T]) -> Vec<(&T, &T)> {
        let mut pairs: Vec<(&T, &T)> = vec![];
        for left in collection.iter().enumerate() {
            for right in collection.iter().enumerate() {
                if left.0 < right.0 {
                    pairs.push((&left.1, &right.1))
                }
            }
        }

        pairs
    }

    pub mod image {
        use std::collections::HashSet;

        use crate::geometry::Coordinate;

        const GALAXY: char = '#';
        //const EMPTY_SPACE: char = '.';

        #[derive(Debug)]
        pub struct Image {
            pixels: Vec<Vec<char>>,
        }

        impl Image {
            pub fn new(lines: &[&str]) -> Image {
                Image {
                    pixels: lines.iter().map(|l| l.chars().collect()).collect(),
                }
            }

            pub fn galaxies(&self, expansion_factor: i32) -> Vec<Coordinate> {
                if expansion_factor < 1 {
                    panic!("expansion factor must be 1 or more")
                }

                let mut non_empty_rows = HashSet::<usize>::new();
                let mut non_empty_columns = HashSet::<usize>::new();
                for (y, line) in self.pixels.iter().enumerate() {
                    for (x, pixel) in line.iter().enumerate() {
                        if *pixel == GALAXY {
                            non_empty_rows.insert(y);
                            non_empty_columns.insert(x);
                        }
                    }
                }

                let mut galaxies = Vec::<Coordinate>::new();

                let mut y_expanded: i64 = 0;
                for (y, line) in self.pixels.iter().enumerate() {
                    if !non_empty_rows.contains(&y) {
                        y_expanded += expansion_factor as i64;
                        continue;
                    }

                    let mut x_expanded: i64 = 0;
                    for (x, char) in line.iter().enumerate() {
                        if !non_empty_columns.contains(&x) {
                            x_expanded += expansion_factor as i64;
                            continue;
                        }

                        if *char == GALAXY {
                            galaxies.push(Coordinate {
                                x: x_expanded,
                                y: y_expanded,
                            })
                        }

                        x_expanded += 1;
                    }

                    y_expanded += 1;
                }

                galaxies
            }
        }
    }
}

// move to common
mod geometry {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Coordinate {
        pub x: i64,
        pub y: i64,
    }

    pub fn calculate_path_distance(left: &Coordinate, right: &Coordinate) -> i64 {
        let x_distance = i64::abs(right.x - left.x);
        let y_distance = i64::abs(right.y - left.y);

        // let non_diagonal_component = i64::abs(x_distance - y_distance);
        // let diagonal_component = i64::max(x_distance, y_distance) - non_diagonal_component;

        // non_diagonal_component + diagonal_component

        x_distance + y_distance
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        astronomy::{find_pairs, image::Image, sum_distances_between_galaxy_pairs},
        geometry::Coordinate,
    };

    #[rustfmt::skip]
    const IMAGE: &[&str] = &[
        /*  v  v  v  */
        "...#......",
        ".......#..",
        "#.........",
    /*>*/"..........",/*<*/
        "......#...",
        ".#........",
        ".........#",
    /*>*/"..........",/*<*/
        ".......#..",
        "#...#.....",
        /*   ^  ^  ^ */
    ];

    #[test]
    fn part_1_works() {
        let image = Image::new(IMAGE);
        let answer = sum_distances_between_galaxy_pairs(&image, 2);

        assert_eq!(answer, 374);
    }

    #[test]
    fn part_2_works() {
        let image = Image::new(IMAGE);
        let answer = sum_distances_between_galaxy_pairs(&image, 10);

        assert_eq!(answer, 1030);
    }

    #[test]
    fn can_get_galaxies() {
        let image_lines = vec!["...#......", ".......#..", "#........."];
        let image = Image::new(&image_lines);
        let expected = vec![
            Coordinate { y: 0, x: 3 },
            Coordinate { y: 1, x: 7 },
            Coordinate { y: 2, x: 0 },
        ];

        let galaxies = image.galaxies(1);

        assert_eq!(galaxies, expected);
    }

    #[test]
    fn can_get_galaxies_with_expansion() {
        #[rustfmt::skip]
        let example = vec![
            ". . # . . . . . .",
            ". . . . . . # . .",
            ". . . . . . . . .",
            "# . . . . . . . .",
            ];

        let lines = remove_whitespace(&example);
        let lines_ref: Vec<&str> = lines.iter().map(|x| x.as_ref()).collect();

        println!("{:?}", lines_ref);

        let image = Image::new(&lines_ref);
        let expected = vec![
            Coordinate { y: 0, x: 2 + 9 },
            Coordinate {
                y: 1,
                x: 6 + (9 * 4),
            },
            Coordinate { y: 3 + 9, x: 0 },
        ];

        let galaxies = image.galaxies(10);

        assert_eq!(galaxies, expected);
    }

    fn remove_whitespace(lines: &[&str]) -> Vec<String> {
        lines
            .iter()
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect()
    }

    #[test]
    fn can_get_pairs() {
        let items = vec![5, 6, 7, 8];
        let expected = vec![(5, 6), (5, 7), (5, 8), (6, 7), (6, 8), (7, 8)];

        let pairs = find_pairs(&items);

        assert_eq!(
            pairs,
            expected
                .iter()
                .map(|(l, r)| (l, r))
                .collect::<Vec<(&i32, &i32)>>()
        );
    }
}

#[cfg(test)]
mod geometry_tests {
    use crate::geometry::{calculate_path_distance, Coordinate};
    use test_case::test_case;

    #[test_case(Coordinate { x: 5, y: 7 }, Coordinate { x: 5, y: 7 }, 0; "itself")]
    #[test_case(Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }, 1; "1 down")]
    #[test_case(Coordinate { x: 0, y: 2 }, Coordinate { x: 0, y: 1 }, 1; "1 up")]
    #[test_case(Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 0 }, 1; "1 right")]
    #[test_case(Coordinate { x: 2, y: 0 }, Coordinate { x: 1, y: 0 }, 1; "1 left")]
    #[test_case(Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 1 }, 2; "1 down and right")]
    #[test_case(Coordinate { x: 2, y: 2 }, Coordinate { x: 0, y: 0 }, 4; "2 up and left")]
    fn can_get_distance_between_coordinates(left: Coordinate, right: Coordinate, expected: i64) {
        let distance = calculate_path_distance(&left, &right);

        assert_eq!(distance, expected);
    }
}
