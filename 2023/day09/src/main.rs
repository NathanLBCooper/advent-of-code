use common::file;

use crate::oasis::{parse, part_1, part_2};

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let sequences = parse(&lines);

    println!("part 1: {}", part_1(&sequences));
    println!("part 2: {}", part_2(&sequences));
}

mod oasis {
    // could remove double work calculate_diff_layers, and not bother finding the middle values etc

    pub fn part_1(sequences: &Vec<Vec<i32>>) -> i32 {
        sequences
            .iter()
            .fold(0, |acc, n| acc + infer_next_number(n))
    }

    pub fn part_2(sequences: &Vec<Vec<i32>>) -> i32 {
        sequences
            .iter()
            .fold(0, |acc, n| acc + infer_preceeding_number(n))
    }

    pub fn infer_preceeding_number(sequence: &[i32]) -> i32 {
        let rev: Vec<i32> = sequence.iter().rev().map(|v| *v).collect();

        infer_next_number(&rev)
    }

    pub fn infer_next_number(sequence: &[i32]) -> i32 {
        let layers = calculate_diff_layers(sequence);

        /*
           for given example
           (2 + 4 + 9) + 30 = 45, which is the next answer
        */
        let diff = layers.iter().fold(0, |acc, f| acc + f.last().unwrap());

        sequence.last().unwrap() + diff
    }

    fn calculate_diff_layers(sequence: &[i32]) -> Vec<Vec<i32>> {
        let mut last_layer = sequence;
        let mut layers: Vec<Vec<i32>> = Vec::new();
        loop {
            /*
                eg for
            10  13  16  21  30
                find:
              3   3   5   9
                0   2   4
                  2   2
                    0
                */
            let layer = calculate_diffs(&last_layer);
            if layer.iter().all(|n| *n == 0) {
                break;
            }

            layers.push(layer);
            last_layer = &layers.last().unwrap();
        }

        layers
    }

    fn calculate_diffs(sequence: &[i32]) -> Vec<i32> {
        let mut diffs: Vec<i32> = Vec::new();
        for i in 0..sequence.len() - 1 {
            let diff = sequence[i + 1] - sequence[i];
            diffs.push(diff);
        }

        diffs
    }

    pub fn parse_sequence(line: &str) -> Vec<i32> {
        line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    pub fn parse(lines: &Vec<String>) -> Vec<Vec<i32>> {
        lines.iter().map(|l| parse_sequence(l)).collect()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::oasis;

    #[test_case(vec![1,2,3,4], 5)]
    #[test_case(vec![6,56,106], 156)]
    fn infer_next_number_works_for_linear(sequence: Vec<i32>, expected: i32) {
        assert_eq!(oasis::infer_next_number(&sequence), expected);
    }

    #[test_case(vec![1,3,6,10,15,21], 28)]
    fn infer_next_number_works_for_geometric(sequence: Vec<i32>, expected: i32) {
        assert_eq!(oasis::infer_next_number(&sequence), expected);
    }

    #[test_case(vec![1,2,3,4], 0)]
    #[test_case(vec![-1,0,1,2,3,4], -2)]
    #[test_case(vec![6,56,106], -44)]
    fn infer_preceeding_number_works_for_linear(sequence: Vec<i32>, expected: i32) {
        assert_eq!(oasis::infer_preceeding_number(&sequence), expected);
    }

    #[test_case(vec![10,13,16,21,30,45], 5)]
    #[test_case(vec![5,10,13,16,21,30,45], -4)]
    fn infer_preceeding_number_works_for_geometric(sequence: Vec<i32>, expected: i32) {
        assert_eq!(oasis::infer_preceeding_number(&sequence), expected);
    }

    #[test_case(vec![13, 14, 13, 10, 5, -2, -11, -22, -35, -50, -67, -86, -107, -130, -155, -182, -211, -242, -275, -310, -347], 10)]
    fn foo(sequence: Vec<i32>, expected: i32) {
        assert_eq!(oasis::infer_preceeding_number(&sequence), expected);
    }
}
