use common::file;

use crate::pipe_maze::parse;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    // No zero cost convertion from &[&str] to Vec<String>. Maybe &[&str] is a poor choice of signature?
    let v: Vec<&str> = lines.iter().map(|x| x.as_ref()).collect();

    let maze: pipe_maze::Maze = parse(&v);

    println!("part 1: {}", maze.distance_to_pipe_furthest_from_start());
    println!("part 2: {}", "todo");
}

mod pipe_maze {
    use std::{
        collections::{HashMap, HashSet},
        ops::Add,
    };

    pub struct Maze {
        pipes: HashMap<Coordinate, Pipe>,
    }

    #[derive(Debug)]
    struct Path {
        end: (Coordinate, Pipe),
        length: i32,
    }

    impl Maze {
        pub fn distance_to_pipe_furthest_from_start(&self) -> i32 {
            let start = self
                .pipes
                .iter()
                .find(|(c, p)| **p == Pipe::Start)
                .map(|(c, p)| (c.clone(), p.clone()))
                .unwrap();

            let paths = self.search_for_paths(start);

            paths.iter().map(|p| p.length).max().unwrap()
        }

        fn search_for_paths(&self, start: (Coordinate, Pipe)) -> Vec<Path> {
            let mut visited: HashSet<Coordinate> = HashSet::new();

            let mut search_level = vec![Path {
                end: (start.0.clone(), start.1.clone()),
                length: 0,
            }];

            // Do breadth first search of all possible paths, until unable to make progress into un-visited coordinate
            loop {
                let mut next_level: Vec<Path> = vec![];
                for path in search_level.iter() {
                    let next_paths: Vec<Path> = self.continue_path_one_step(path, &mut visited);
                    if next_paths.is_empty() {
                        continue;
                    }

                    next_level.extend(next_paths);
                }

                if next_level.is_empty() {
                    break;
                }

                search_level = next_level;
            }

            search_level
        }

        fn continue_path_one_step(
            &self,
            path: &Path,
            visited: &mut HashSet<Coordinate>,
        ) -> Vec<Path> {
            let directions_from_current = directions(&path.end.1);
            let accessable_coordinates = directions_from_current.iter().filter_map(|direction| {
                let accessable: Coordinate = direction + &path.end.0;
                match self.pipes.get(&accessable) {
                    Some(p) => Some((accessable.clone(), p.clone())),
                    None => None,
                }
            });

            let next_coordinates = accessable_coordinates
                .filter(|(c, p)| {
                    let directions_from_adjacent = directions(&p); // todo double work. Actually more than double. Optimize by storing against enum somehow?
                    let is_connected = directions_from_adjacent.iter().any(|d| c + d == path.end.0);
                    is_connected
                })
                .filter(|(c, _)| !visited.contains(&c))
                .collect::<Vec<(Coordinate, Pipe)>>();

            let next_paths: Vec<Path> = next_coordinates
                .iter()
                .map(|(c, p)| {
                    let path = Path {
                        end: (c.clone(), p.clone()),
                        length: path.length + 1,
                    };
                    visited.insert(path.end.0.clone());
                    path
                })
                .collect();

            next_paths
        }
    }

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    impl Add for &Coordinate {
        type Output = Coordinate;
        fn add(self, rhs: Self) -> Coordinate {
            Coordinate {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Coordinate {
        pub fn up() -> Coordinate {
            Coordinate { x: 0, y: -1 }
        }
        pub fn down() -> Coordinate {
            Coordinate { x: 0, y: 1 }
        }
        pub fn left() -> Coordinate {
            Coordinate { x: -1, y: 0 }
        }
        pub fn right() -> Coordinate {
            Coordinate { x: 1, y: 0 }
        }
    }

    #[derive(PartialEq, Clone, Debug)]
    pub enum Pipe {
        Vertical,   // ↑
        Horizontal, // ↓
        NorthEast,  // ↳
        NorthWest,  // ↲
        SouthWest,  // ↰
        SouthEast,  // ↱
        Start,
    }

    fn directions(pipe: &Pipe) -> Vec<Coordinate> {
        match pipe {
            Pipe::Vertical => vec![Coordinate::up(), Coordinate::down()],
            Pipe::Horizontal => vec![Coordinate::left(), Coordinate::right()],
            Pipe::NorthEast => vec![Coordinate::up(), Coordinate::right()],
            Pipe::NorthWest => vec![Coordinate::up(), Coordinate::left()],
            Pipe::SouthWest => vec![Coordinate::down(), Coordinate::left()],
            Pipe::SouthEast => vec![Coordinate::down(), Coordinate::right()],
            Pipe::Start => vec![
                Coordinate::up(),
                Coordinate::down(),
                Coordinate::left(),
                Coordinate::right(),
            ],
        }
    }

    pub fn parse(lines: &[&str]) -> Maze {
        let mut hash_map: HashMap<Coordinate, Pipe> = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match parse_pipe(char) {
                    Some(p) => {
                        hash_map.insert(
                            Coordinate {
                                x: x as i32,
                                y: y as i32,
                            },
                            p,
                        );
                    }
                    None => (),
                };
            }
        }

        Maze { pipes: hash_map }
    }

    fn parse_pipe(char: char) -> Option<Pipe> {
        match char {
            '|' => Some(Pipe::Vertical),
            '-' => Some(Pipe::Horizontal),
            'L' => Some(Pipe::NorthEast),
            'J' => Some(Pipe::NorthWest),
            '7' => Some(Pipe::SouthWest),
            'F' => Some(Pipe::SouthEast),
            'S' => Some(Pipe::Start),
            '.' => None,
            _ => panic!("Can't parse pipe"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe_maze::parse;

    #[test]
    fn works_for_basic_example() {
        #[rustfmt::skip]
        let square_loop = vec![".....",
                                          ".S-7.",
                                          ".|.|.",
                                          ".L-J.",
                                          "....."];

        let maze = parse(&square_loop);

        let distance = maze.distance_to_pipe_furthest_from_start();

        assert_eq!(distance, 4);
    }
}
