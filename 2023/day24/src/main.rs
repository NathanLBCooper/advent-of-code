fn main() {
    println!("Hello, world!");
}

mod hail {
    pub struct HailStone {
        pub initial_position: Vector3D,
        pub velocity: Vector3D,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Vector3D {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    #[derive(Clone, Debug)]
    pub struct Vector2Df {
        pub x: f64,
        pub y: f64,
    }

    impl Vector2Df {
        pub fn approximate_eq(self, rhs: Self) -> bool {
            const EPSILON: f64 = 0.00001;
            (self.x - rhs.x).abs() < EPSILON && (self.y - rhs.y).abs() < EPSILON
        }
    }

    pub fn approximate_eq(left: f64, right: f64) -> bool {
        const EPSILON: f64 = 0.00001;
        (left - right).abs() < EPSILON
    }

    struct Dimension {
        pub initial_position: i64,
        pub velocity: i64,
    }

    pub fn calculate_xy_collision(left: &HailStone, right: &HailStone) -> Option<Vector2Df> {
        let left_dimensions = expand_dimensions(left);
        let right_dimensions = expand_dimensions(right);

        let mut collision_time: Option<f64> = None;
        for i in left_dimensions.iter().zip(right_dimensions.iter()) {
            let dimension_collision_time = calculate_collision_time(i.0, i.1);

            println!("{:?}", dimension_collision_time);
            match dimension_collision_time {
                None => return None,
                Some(t) => match collision_time {
                    None => collision_time = dimension_collision_time,
                    Some(last) => {
                        if !approximate_eq(last, t) {
                            return None;
                        }
                    }
                },
            }
        }

        let t = collision_time.unwrap();

        Some(Vector2Df {
            x: left.initial_position.x as f64 + left.velocity.x as f64 * t,
            y: left.initial_position.y as f64 + left.velocity.y as f64 * t,
        })
    }

    fn calculate_collision_time(left: &Dimension, right: &Dimension) -> Option<f64> {
        let relative_velocity: i64 = right.velocity - left.velocity;

        println!("{:?}", left.initial_position - right.initial_position);
        println!("{:?}", relative_velocity);

        match relative_velocity {
            0 => todo!(), // if same intital_position they collide at all times
            _ => Some(
                (left.initial_position - right.initial_position) as f64 / relative_velocity as f64,
            ),
        }
    }

    fn expand_dimensions(hailStone: &HailStone) -> [Dimension; 3] {
        [
            Dimension {
                initial_position: hailStone.initial_position.x,
                velocity: hailStone.velocity.x,
            },
            Dimension {
                initial_position: hailStone.initial_position.y,
                velocity: hailStone.velocity.y,
            },
            Dimension {
                initial_position: hailStone.initial_position.y,
                velocity: hailStone.velocity.y,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::hail::*;
    use test_case::test_case;

    #[test]
    fn foo() {}

    #[test_case(
        HailStone { initial_position: Vector3D { x: 19, y: 13, z:30 }, velocity: Vector3D { x: -2, y: 1, z:-2 }},
        HailStone { initial_position: Vector3D { x: 18, y: 19, z:22 }, velocity: Vector3D { x: -1, y: -1, z:-2 }},
        Vector2Df { x: 14.333, y: 15.333 }
    )]
    fn can_calculate_crossing_hailstones(
        left: HailStone,
        right: HailStone,
        expected_collision: Vector2Df,
    ) {
        let collision = calculate_xy_collision(&left, &right);

        assert!(collision.is_some());
        assert!(collision.unwrap().approximate_eq(expected_collision));
    }
}
