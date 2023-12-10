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
        clone,
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
            let mut visited: HashSet<Coordinate> = HashSet::new();
            let start = self
                .pipes
                .iter()
                .find(|(c, p)| **p == Pipe::Start)
                .map(|(c, p)| Path {
                    end: (c.clone(), p.clone()),
                    length: 0,
                })
                .unwrap();

            let mut heads = vec![start];
            loop {
                //println!("loop {:?}", heads);

                let mut next_heads: Vec<Path> = vec![];
                for head in heads.iter() {
                    let next = self.take_step(&head, &mut visited);
                    if next.is_empty() {
                        continue;
                    }

                    next_heads.extend(next);
                }

                if next_heads.is_empty() {
                    break;
                }

                heads = next_heads;
            }

            heads.iter().map(|p| p.length).max().unwrap()
        }

        fn take_step(&self, head: &Path, visited: &mut HashSet<Coordinate>) -> Vec<Path> {
            let directions_from_current = directions(&head.end.1);
            //println!("directions_from_current {:?}", directions_from_current);

            let adjacent = directions_from_current.iter().filter_map(|d| {
                let accessable: Coordinate = d + &head.end.0;
                match self.pipes.get(&accessable) {
                    Some(p) => Some((accessable, p.clone())),
                    None => None,
                }
            });

            // println!(
            //     "adjacent {:?}",
            //     adjacent.clone().collect::<Vec<(Coordinate, Pipe)>>()
            // );

            let connected = adjacent.filter_map(|(c, p)| {
                let directions_from_adjacent = directions(&p); // todo double work
                for d in directions_from_adjacent {
                    if &c + &d == head.end.0 {
                        return Some((c.clone(), p.clone()));
                    }
                }

                return None;
            });

            // println!(
            //     "connected {:?}",
            //     connected.clone().collect::<Vec<(Coordinate, Pipe)>>()
            // );

            let unvisited = connected
                .filter_map(|(c, p)| match visited.contains(&c) {
                    true => None,
                    false => Some((c.clone(), p.clone())),
                })
                .collect::<Vec<(Coordinate, Pipe)>>();

            //println!("unvisited {:?}", unvisited);

            let next: Vec<Path> = unvisited
                .iter()
                .map(|(c, p)| {
                    let path = Path {
                        end: (c.clone(), p.clone()),
                        length: head.length + 1,
                    };
                    visited.insert(path.end.0.clone());
                    path
                })
                .collect();

            //println!("next {:?}", next);

            next
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
