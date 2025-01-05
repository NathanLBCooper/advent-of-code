use common::core::AdventError;
use common::file::read_lines;

fn main() {
    let matrix = parse_file().unwrap();
    println!("Part 1 Sum: {:?}", part_one(&matrix));
    println!("Part 2 Sum: {:?}", part_two(&matrix));
}

fn parse_file() -> Result<Matrix, AdventError> {
    let lines = read_lines("./day04/input.txt")?;

    let data: Vec<Vec<char>> = lines.iter().map(|l| parse_line(l)).collect();

    let size = data[0].len() as i32;
    Ok(Matrix { data, size })
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn part_one(matrix: &Matrix) -> usize {
    let mut matches = 0;
    for y in 0..matrix.size {
        for x in 0..matrix.size {
            matches += search_from((x, y).into(), &matrix);
        }
    }

    matches
}

fn part_two(matrix: &Matrix) -> usize {
    let mut matches = 0;
    for y in 0..matrix.size {
        for x in 0..matrix.size {
            if search_from_two((x, y).into(), &matrix) {
                matches += 1;
            }
        }
    }

    matches
}

fn search_from(start: Vector, matrix: &Matrix) -> usize {
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

    if matrix.get(&start) != Some(WORD[0]) {
        return 0;
    }

    #[rustfmt::skip]
    let directions: [Vector; 8] = [
        (-1, -1).into(), (0, -1).into(), ( 1, -1).into(),
        (-1,  0).into(),                 ( 1,  0).into(),
        (-1,  1).into(), (0,  1).into(), ( 1,  1).into(),
    ];

    let candidates: Vec<Line> = directions
        .iter()
        .map(|d| Line {
            direction: d.clone(),
            position: start.clone(),
        })
        .collect();

    let mut sum = 0;
    for candidate in candidates {
        let mut success = true;
        for (i, &c) in WORD[1..].iter().enumerate() {
            let point = candidate.position.clone() + (candidate.direction.clone() * (i as i32 + 1));

            if matrix.get(&point) != Some(c) {
                success = false;
                break;
            }
        }

        if success {
            sum += 1;
        }
    }

    sum
}

fn search_from_two(start: Vector, matrix: &Matrix) -> bool {
    const WORD: [char; 3] = ['M', 'A', 'S'];

    if matrix.get(&start) != Some(WORD[1]) {
        return false;
    }

    // /
    let first: [Vector; 2] = [
        start.clone() + (-1, 1).into(),
        start.clone() + (1, -1).into(),
    ];
    // \
    let second: [Vector; 2] = [
        start.clone() + (-1, -1).into(),
        start.clone() + (1, 1).into(),
    ];

    if (matrix.get(&first[0]) == Some(WORD[0]) && matrix.get(&first[1]) == Some(WORD[2])
        || matrix.get(&first[0]) == Some(WORD[2]) && matrix.get(&first[1]) == Some(WORD[0]))
        && (matrix.get(&second[0]) == Some(WORD[0]) && matrix.get(&second[1]) == Some(WORD[2])
            || matrix.get(&second[0]) == Some(WORD[2]) && matrix.get(&second[1]) == Some(WORD[0]))
    {
        return true;
    }

    false
}

#[derive(Debug)]
struct Matrix {
    size: i32,
    data: Vec<Vec<char>>,
}

impl Matrix {
    fn get(&self, coordinate: &Vector) -> Option<char> {
        if coordinate.x < 0
            || coordinate.y < 0
            || coordinate.x >= self.size
            || coordinate.y >= self.size
        {
            return None;
        }

        return Some(self.data[coordinate.y as usize][coordinate.x as usize]);
    }
}

#[derive(Clone, Debug)]
struct Line {
    position: Vector,
    direction: Vector,
}

#[derive(Clone, Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Vector {
    fn from(tuple: (i32, i32)) -> Self {
        Vector {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: i32::from(self.x) + other.x,
            y: i32::from(self.y) + other.y,
        }
    }
}

impl std::ops::Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: i32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
