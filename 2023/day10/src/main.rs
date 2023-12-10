use common::file;

use crate::pipe_maze::parse;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    // No zero cost convertion from &[&str] to Vec<String>. Maybe &[&str] is a poor choice of signature?
    let v: Vec<&str> = lines.iter().map(|x| x.as_ref()).collect();

    let maze: pipe_maze::Maze = parse(&v);

    println!("part 1: {}", maze.distance_to_pipe_furthest_from_start());
    println!("part 2: {}", maze.count_enclosed_area());
}

mod pipe_maze {
    use colored::Colorize;
    use std::{
        collections::{HashMap, HashSet},
        ops::Add,
    };

    pub struct Maze {
        height: i32,
        width: i32,
        pipes: HashMap<Coordinate, Pipe>,
    }

    #[derive(Debug)]
    struct Path {
        end: (Coordinate, Pipe),
        visited: HashSet<Coordinate>,
        length: i32,
    }

    impl Maze {
        pub fn distance_to_pipe_furthest_from_start(&self) -> i32 {
            let paths = self.search_for_paths();

            paths.iter().map(|p| p.length).max().unwrap()
        }

        pub fn count_enclosed_area(&self) -> i32 {
            let paths = self.search_for_paths();
            // Can't be bothered to look for loops. Maybe it's the last remaining (ie longest) path?
            if paths.len() > 1 {
                panic!("I'm guessing theres one longest path");
            }
            // nope, doesn't work

            let visited = &paths.first().unwrap().visited;

            let mut fill_stack: Vec<Coordinate> = vec![];
            let mut filled: HashSet<Coordinate> = HashSet::new();
            self.insert_unvisited_edges(&visited, &mut fill_stack);

            // flood fill
            let area: i32 = self.area();
            let mut counter = 0;
            loop {
                let node = match fill_stack.pop() {
                    Some(n) => n,
                    None => break,
                };

                let should_fill =
                    self.in_bounds(&node) && !visited.contains(&node) && !filled.contains(&node);
                if !should_fill {
                    continue;
                }

                filled.insert(node.clone());

                let next = [
                    &node + &Coordinate::up(),
                    &node + &Coordinate::right(),
                    &node + &Coordinate::down(),
                    &node + &Coordinate::left(),
                ];

                fill_stack.extend(next);

                counter += 1;
                if counter > area {
                    panic!("{:?}", fill_stack);
                }
            }

            let filled_count = filled.iter().count() as i32;
            let visited_count = visited.iter().count() as i32;

            let enclosed_count = self.area() - filled_count - visited_count;

            self.display(&visited, &filled);

            enclosed_count
        }

        fn area(&self) -> i32 {
            self.width * self.height
        }

        fn in_bounds(&self, coordinate: &Coordinate) -> bool {
            coordinate.x >= 0
                && coordinate.y >= 0
                && coordinate.x < self.width
                && coordinate.y < self.height
        }

        pub fn display(&self, visited: &HashSet<Coordinate>, filled: &HashSet<Coordinate>) {
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord = Coordinate { x, y };

                    let str = match self.pipes.get(&coord) {
                        Some(p) => display_pipe(p),
                        None => String::from("."),
                    };

                    if visited.contains(&coord) {
                        print!("{}", str.red());
                    } else if filled.contains(&coord) {
                        print!("{}", str.green());
                    } else {
                        print!("{}", str.white());
                    }
                    print!(" ");
                }
                println!();
            }
        }

        fn insert_unvisited_edges(
            &self,
            visited: &HashSet<Coordinate>,
            fill_stack: &mut Vec<Coordinate>,
        ) {
            // top and bottom
            for y in [0, self.height - 1] {
                for x in 0..self.width {
                    let c = Coordinate { x, y };
                    match visited.get(&c) {
                        Some(_) => (),
                        None => fill_stack.push(c),
                    }
                }
            }

            // remaining sides
            for y in 1..(self.height - 1) {
                for x in [0, self.width - 1] {
                    let c = Coordinate { x, y };
                    match visited.get(&c) {
                        Some(_) => (),
                        None => fill_stack.push(c),
                    }
                }
            }
        }

        fn search_for_paths(&self) -> Vec<Path> {
            let start = self
                .pipes
                .iter()
                .find(|(c, p)| **p == Pipe::Start)
                .map(|(c, p)| (c.clone(), p.clone()))
                .unwrap();

            let mut visited: HashSet<Coordinate> = HashSet::new();

            let mut start_path = Path {
                end: (start.0.clone(), start.1.clone()),
                visited: HashSet::new(),
                length: 0,
            };
            start_path.visited.insert(start.0.clone());

            let mut search_level = vec![start_path];

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
                    let mut new_path: Path = Path {
                        end: (c.clone(), p.clone()),
                        visited: path.visited.clone(),
                        length: path.length + 1,
                    };
                    new_path.visited.insert(new_path.end.0.clone());
                    visited.insert(new_path.end.0.clone());
                    new_path
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
        Horizontal, // →
        NorthEast,  // ↳
        NorthWest,  // ↲
        SouthWest,  // ↰
        SouthEast,  // ↱
        Start,
    }

    fn display_pipe(pipe: &Pipe) -> String {
        let s = match pipe {
            Pipe::Vertical => "↑",
            Pipe::Horizontal => "→",
            Pipe::NorthEast => "↳",
            Pipe::NorthWest => "↲",
            Pipe::SouthWest => "↰",
            Pipe::SouthEast => "↱",
            Pipe::Start => "S",
        };

        String::from(s)
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
        let mut width: Option<usize> = None;
        for (y, line) in lines.iter().enumerate() {
            match width {
                Some(_) => (),
                None => width = Some(line.len()),
            }
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

        Maze {
            pipes: hash_map,
            height: lines.len() as i32,
            width: width.unwrap() as i32,
        }
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
    fn can_find_distance_to_pipe_furthest_from_start() {
        #[rustfmt::skip]
        let square_loop = vec![
            ".....",
            ".S-7.",
            ".|.|.",
            ".L-J.",
            "....."
        ];

        let maze = parse(&square_loop);

        let distance = maze.distance_to_pipe_furthest_from_start();

        assert_eq!(distance, 4);
    }

    #[test]
    fn can_find_enclosed_area() {
        let example = vec![
            "...........",
            ".S-------7.",
            ".|F-----7|.",
            ".||.....||.",
            ".||.....||.",
            ".|L-7.F-J|.",
            ".|..|.|..|.", // the ones marked I are in the loop: .|II|O|II|.
            ".L--J.L--J.",
            "...........",
        ];

        let maze = parse(&example);

        let count = maze.count_enclosed_area();

        assert_eq!(count, 4);
    }

    #[test]
    fn can_find_enclosed_area_for_larger_example() {
        let example = vec![
            ".F----7F7F7F7F-7....",
            ".|F--7||||||||FJ....",
            ".||.FJ||||||||L7....",
            "FJL7L7LJLJ||LJ.L-7..",
            "L--J.L7...LJS7F-7L7.",
            "....F-J..F7FJ|L7L7L7",
            "....L7.F7||L7|.L7L7|",
            ".....|FJLJ|FJ|F7|.LJ",
            "....FJL-7.||.||||...",
            "....L---J.LJ.LJLJ...",
        ];

        let maze = parse(&example);

        let count = maze.count_enclosed_area(); // the problem is I haven't found the "loop". I found everywhere that was accessable

        assert_eq!(count, 8);
    }

    #[test]
    fn can_find_enclosed_area_with_junk_pipess() {
        let example = vec![
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ];

        let maze = parse(&example);

        let count = maze.count_enclosed_area();

        assert_eq!(count, 10);
    }
}
