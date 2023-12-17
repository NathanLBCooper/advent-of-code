use crate::hot_springs::Map;
use common::file;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();
    let v: Vec<&str> = lines.iter().map(|x| x.as_ref()).collect();

    let map = Map::new(&v);

    println!("part 1: {}", map.part_1());
}

mod hot_springs {
    use itertools::Itertools;

    pub struct Map {
        rows: Vec<Row>,
    }

    impl Map {
        pub fn new(lines: &[&str]) -> Map {
            let rows = lines.iter().map(|l| Row::new(l)).collect();
            Map { rows }
        }

        pub fn part_1(&self) -> i32 {
            self.rows
                .iter()
                .map(|r| r.calculate_possible_arrangements())
                .sum()
        }
    }

    pub struct Row {
        springs: Vec<Option<Condition>>,
        group_sizes: Vec<i32>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Condition {
        Operational,
        Damaged,
    }

    impl Row {
        pub fn new(line: &str) -> Row {
            let mut iter = line.split_whitespace();
            let springs = iter
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Some(Condition::Operational),
                    '#' => Some(Condition::Damaged),
                    '?' => None,
                    _ => panic!("not a valid condition"),
                })
                .collect();
            let group_sizes = iter
                .next()
                .unwrap()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

            Row {
                springs,
                group_sizes,
            }
        }

        pub fn calculate_possible_arrangements(&self) -> i32 {
            let unknown_count = self.springs.iter().filter(|s| s.is_none()).count();

            let substituations = (0..unknown_count)
                .map(|_| [Condition::Operational, Condition::Damaged])
                .multi_cartesian_product();

            let all_possible_spring_arrangements = substituations.map(|sub| {
                Self::apply_substitutions(
                    &self.springs,
                    &sub.iter().cloned().collect::<Vec<Condition>>(),
                )
            });

            let count_of_valid_possibles = all_possible_spring_arrangements
                .filter(|s| Self::is_valid(&s, &self.group_sizes))
                .count();

            count_of_valid_possibles as i32
        }

        pub fn is_valid(springs: &[Condition], group_sizes: &[i32]) -> bool {
            let grouped_by_condition = springs.iter().group_by(|s| *s);
            let inferred_group_sizes =
                grouped_by_condition
                    .into_iter()
                    .filter_map(|(condition, group)| {
                        if *condition == Condition::Damaged {
                            Some(group.count() as i32)
                        } else {
                            None
                        }
                    });

            inferred_group_sizes.eq(group_sizes.iter().copied())
        }

        fn apply_substitutions(
            springs: &[Option<Condition>],
            substitution: &[Condition],
        ) -> Vec<Condition> {
            let mut result = Vec::<Condition>::new();
            let mut cnt = 0;
            for spring in springs {
                match spring {
                    Some(s) => result.push(s.clone()),
                    None => {
                        result.push(substitution[cnt].clone());
                        cnt += 1;
                    }
                }
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hot_springs::{Map, Row};
    use test_case::test_case;

    const EXAMPLE: &[&str] = &[
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn part_1_works() {
        let map = Map::new(EXAMPLE);
        let answer = map.part_1();

        assert_eq!(answer, 21);
    }

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 4)]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[test_case("????.#...#... 4,1,1", 1)]
    #[test_case("????.######..#####. 1,6,5", 4)]
    #[test_case("?###???????? 3,2,1", 10)]
    fn can_calculate_possible_arrangements_for_row(line: &str, expected: i32) {
        let row = Row::new(line);

        let result = row.calculate_possible_arrangements();
        assert_eq!(result, expected);
    }
}
