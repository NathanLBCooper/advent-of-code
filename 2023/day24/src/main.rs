use common::file;
use hail::{calculate_xy_intersections, parse};

fn main() {
    let file = String::from("./input.txt");
    let input_lines = file::read_lines(&file).unwrap();
    let v: Vec<&str> = input_lines.iter().map(|x| x.as_ref()).collect();

    let hailstones = parse(&v);

    let intersections = calculate_xy_intersections(&hailstones, 200000000000000, 400000000000000);

    println!("{:?}", intersections.len());
}

mod geometry_2d {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Vector2D {
        pub x: i64,
        pub y: i64,
    }

    #[derive(Clone, Debug)]
    pub struct Vector2Df {
        pub x: f64,
        pub y: f64,
    }

    impl Vector2Df {
        pub fn approximate_eq(&self, rhs: &Self, epsilon: f64) -> bool {
            (self.x - rhs.x).abs() < epsilon && (self.y - rhs.y).abs() < epsilon
        }
    }

    #[derive(Clone, Debug)]
    pub struct Line2D {
        /* Line defined between two points as `r = OA + λ * AB` */
        pub vector_0a: Vector2D,
        pub vector_ab: Vector2D,
    }

    impl Line2D {
        // Assuming infinite lines
        pub fn calculate_intersection(&self, rhs: &Self) -> Option<Intersection2D> {
            /*
                `self.vector_0a + λ * self.vector_ab = rhs.vector_0a + μ * rhs.vector_ab` is true at intersection
                This results in a simultaneous equation across the two dimensions (x and y) with two unknowns (λ and μ)
                Finding λ by eliminating μ:
            */
            let intersection_scalar = ((rhs.vector_ab.y * rhs.vector_0a.x)
                - (rhs.vector_ab.y * self.vector_0a.x)
                - (rhs.vector_ab.x * rhs.vector_0a.y)
                + (rhs.vector_ab.x * self.vector_0a.y))
                as f64
                / ((rhs.vector_ab.y * self.vector_ab.x) - (rhs.vector_ab.x * self.vector_ab.y))
                    as f64;

            if intersection_scalar.is_infinite() {
                return None;
            }

            if intersection_scalar.is_nan() {
                return Some(Intersection2D::Line);
            }

            let intersection = Vector2Df {
                x: self.vector_0a.x as f64 + intersection_scalar * self.vector_ab.x as f64,
                y: self.vector_0a.y as f64 + intersection_scalar * self.vector_ab.y as f64,
            };

            Some(Intersection2D::Point(intersection))
        }
    }

    #[derive(Clone, Debug)]
    pub enum Intersection2D {
        Point(Vector2Df),
        Line,
    }
}

#[cfg(test)]
mod geometry_2d_tests {
    use crate::geometry_2d::*;
    use test_case::test_case;

    const EPSILON: f64 = 0.001;

    #[test_case(
        Line2D { vector_0a: Vector2D { x: 19, y: 13 }, vector_ab: Vector2D { x: -2, y: 1 }},
        Line2D { vector_0a: Vector2D { x: 18, y: 19 }, vector_ab: Vector2D { x: -1, y: -1 }},
        Vector2Df { x: 14.333, y: 15.333 }
    )]
    fn can_calculate_crossing_lines(left: Line2D, right: Line2D, expected_intersection: Vector2Df) {
        let result = left.calculate_intersection(&right);

        let intersection = result.unwrap();
        match intersection {
            Intersection2D::Point(p) => {
                assert!(p.approximate_eq(&expected_intersection, EPSILON));
            }
            Intersection2D::Line => panic!("expected Point"),
        }
    }

    #[test_case(
        Line2D { vector_0a: Vector2D { x: 19, y: 13 }, vector_ab: Vector2D { x: -2, y: 1 }},
        Line2D { vector_0a: Vector2D { x: 20, y: 13 }, vector_ab: Vector2D { x: -2, y: 1 }}
    )]
    fn calculates_no_intersection_for_non_intersecting_parallel_lines(left: Line2D, right: Line2D) {
        let result = left.calculate_intersection(&right);

        assert!(result.is_none())
    }

    #[test_case(
        Line2D { vector_0a: Vector2D { x: 19, y: 13 }, vector_ab: Vector2D { x: -2, y: 1 }},
        Line2D { vector_0a: Vector2D { x: 23, y: 11 }, vector_ab: Vector2D { x: -4, y: 2 }}
    )]
    fn can_calculate_intersection_for_parallel_lines(left: Line2D, right: Line2D) {
        let result = left.calculate_intersection(&right);

        let intersection = result.unwrap();
        match intersection {
            Intersection2D::Point(_) => {
                panic!("expected Line");
            }
            Intersection2D::Line => (),
        }
    }
}

mod geometry_3d {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Vector3D {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    #[derive(Clone, Debug)]
    pub struct Line3D {
        /* Line defined between two points as `r = OA + λ * AB` */
        pub vector_0a: Vector3D,
        pub vector_ab: Vector3D,
    }
}

mod hail {
    use crate::{
        geometry_2d::{Intersection2D, Line2D, Vector2D},
        geometry_3d::{Line3D, Vector3D},
    };

    const EPSILON: f64 = 0.001;

    pub fn calculate_xy_intersections(lines: &[Line3D], min: i64, max: i64) -> Vec<Intersection2D> {
        let mut intersections = Vec::<Intersection2D>::new();
        for (i, lhs) in lines.iter().enumerate() {
            for (j, rhs) in lines.iter().enumerate() {
                if i >= j {
                    continue;
                }

                let intersection = calculate_xy_intersection(lhs, rhs, min, max);
                match intersection {
                    Some(v) => intersections.push(v),
                    None => (),
                }
            }
        }

        // bad
        intersections.sort_by_key(|item| match item {
            Intersection2D::Point(p) => ((p.x * 100f64) as i32, (p.y * 100f64) as i32), // very bad
            Intersection2D::Line => (0, 0),
        });
        intersections.dedup_by(
            (|l, r| match l {
                Intersection2D::Point(lp) => match r {
                    Intersection2D::Point(rp) => lp.approximate_eq(rp, 0.001),
                    Intersection2D::Line => false,
                },
                Intersection2D::Line => match r {
                    Intersection2D::Point(_) => false,
                    Intersection2D::Line => true,
                },
            }),
        );

        intersections
    }

    pub fn calculate_xy_intersection(
        lhs: &Line3D,
        rhs: &Line3D,
        min: i64,
        max: i64,
    ) -> Option<Intersection2D> {
        let intersection = reduce_to_2d(lhs).calculate_intersection(&reduce_to_2d(rhs));

        let point = match &intersection {
            Some(i) => match i {
                Intersection2D::Point(p) => p,
                Intersection2D::Line => return intersection,
            },
            None => return intersection,
        };

        let in_bounds = point.x + EPSILON < max as f64
            && point.y + EPSILON < max as f64
            && point.x - EPSILON > min as f64
            && point.y - EPSILON > min as f64;

        if !in_bounds {
            return None;
        }

        intersection
    }

    pub fn reduce_to_2d(line: &Line3D) -> Line2D {
        Line2D {
            vector_0a: Vector2D {
                x: line.vector_0a.x,
                y: line.vector_0a.y,
            },
            vector_ab: Vector2D {
                x: line.vector_ab.x,
                y: line.vector_ab.y,
            },
        }
    }

    pub fn parse(input: &[&str]) -> Vec<Line3D> {
        input.iter().map(|l| parse_line3d(l)).collect()
    }

    fn parse_line3d(input: &str) -> Line3D {
        let parts: Vec<&str> = input.split('@').collect();
        let mut iter = parts.iter();

        Line3D {
            vector_0a: parse_vec3d(iter.next().unwrap()),
            vector_ab: parse_vec3d(iter.next().unwrap()),
        }
    }

    fn parse_vec3d(input: &str) -> Vector3D {
        let numbers: Vec<i64> = input
            .split(',')
            .filter_map(|num| num.trim().parse().ok())
            .collect();
        let mut iter = numbers.iter();

        Vector3D {
            x: *iter.next().unwrap(),
            y: *iter.next().unwrap(),
            z: *iter.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry_2d::*;
    use crate::geometry_3d::*;
    use crate::hail::*;

    const EPSILON: f64 = 0.001;

    #[test]
    fn can_calculate_crossing_hailstones() {
        let left = Line3D {
            vector_0a: Vector3D {
                x: 19,
                y: 13,
                z: 30,
            },
            vector_ab: Vector3D { x: -2, y: 1, z: -2 },
        };
        let right = Line3D {
            vector_0a: Vector3D {
                x: 18,
                y: 19,
                z: 22,
            },
            vector_ab: Vector3D {
                x: -1,
                y: -1,
                z: -2,
            },
        };
        let expected = Vector2Df {
            x: 14.333,
            y: 15.333,
        };

        let result = calculate_xy_intersection(&left, &right, 7, 27);

        let intersection = result.unwrap();
        match intersection {
            Intersection2D::Point(p) => {
                assert!(p.approximate_eq(&expected, EPSILON));
            }
            Intersection2D::Line => panic!("expected Point"),
        }
    }

    #[test]
    fn only_calculates_crossing_hailstones_in_bounds() {
        let left = Line3D {
            vector_0a: Vector3D {
                x: 19,
                y: 13,
                z: 30,
            },
            vector_ab: Vector3D { x: -2, y: 1, z: -2 },
        };
        let right = Line3D {
            vector_0a: Vector3D {
                x: 12,
                y: 31,
                z: 28,
            },
            vector_ab: Vector3D {
                x: -1,
                y: -2,
                z: -2,
            },
        };

        let result = calculate_xy_intersection(&left, &right, 7, 27);

        assert!(result.is_none())
    }

    #[test]
    fn can_calculate_multiple_crossing_hailstones() {
        let one = Line3D {
            vector_0a: Vector3D { x: 0, y: -1, z: 0 },
            vector_ab: Vector3D { x: 1, y: 1, z: 0 },
        };
        let two = Line3D {
            vector_0a: Vector3D { x: -1, y: 0, z: 0 },
            vector_ab: Vector3D { x: 0, y: 1, z: 0 },
        };
        let three = Line3D {
            vector_0a: Vector3D { x: 1, y: -2, z: 0 },
            vector_ab: Vector3D { x: -1, y: 1, z: 0 },
        };

        let arr = [one, two, three];

        let result = calculate_xy_intersections(&arr, -10, 10);

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn can_parse() {
        let lines = [
            "473791829703098, 230456526176051, 354431298438521 @ -186, 25, -20",
            "365401987647919, 199494537264574, 288715138476352 @ -145, 120, 15",
        ];

        let result = parse(&lines);

        assert_eq!(result.len(), 2);
        assert_eq!(result.first().unwrap().vector_0a.x, 473791829703098);
        assert_eq!(result.first().unwrap().vector_ab.x, -186);
        // etc
    }
}
