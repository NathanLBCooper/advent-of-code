use crate::gambling::parse_scratchcards;
use common::file;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let collection = parse_scratchcards(&lines);

    println!("part 1: {}", collection.winnings_part_1().0);

    println!("part 2: {}", collection.winnings_part_2().0);
}

mod gambling {
    use regex::Regex;
    use std::{
        arch::asm,
        collections::{HashMap, HashSet},
        iter::Sum,
        ops::Add,
    };

    pub struct ScratchCardCollection {
        scratch_cards: Vec<ScratchCard>,
    }

    impl ScratchCardCollection {
        pub fn winnings_part_1(&self) -> Point {
            self.scratch_cards.iter().map(|s| s.points()).sum()
        }

        pub fn winnings_part_2(&self) -> Point {
            const STARTING_CARD_COUNT: i32 = 1;
            let mut all_won_cards: HashMap<i32, i32> = self
                .scratch_cards
                .iter()
                .map(|s| (s.id, STARTING_CARD_COUNT))
                .collect();

            for card in self.scratch_cards.iter() {
                let matching = card.number_of_matching_numbers() as i32;
                let card_count = all_won_cards.get(&card.id).unwrap().clone();
                let won_card_ids = (card.id + 1)..=(card.id + matching);

                for card_id in won_card_ids {
                    all_won_cards
                        .entry(card_id)
                        .and_modify(|v| *v += card_count);
                }
            }

            let number_of_won_cards: i32 = all_won_cards.iter().map(|(k, v)| v.clone()).sum();

            return Point(number_of_won_cards);
        }
    }

    pub struct ScratchCard {
        id: i32,
        winning_numbers: Vec<i32>,
        numbers: Vec<i32>,
    }

    impl ScratchCard {
        pub fn id(&self) -> i32 {
            self.id
        }

        pub fn points(&self) -> Point {
            let matching = self.number_of_matching_numbers() as u32;

            match matching == 0 {
                true => Point(0),
                false => Point(i32::pow(2, matching - 1)),
            }
        }

        pub fn number_of_matching_numbers(&self) -> u32 {
            let w: HashSet<i32> = self.winning_numbers.iter().copied().collect();
            let n: HashSet<i32> = self.numbers.iter().copied().collect();
            let matching: u32 = w.intersection(&n).count() as u32;

            matching
        }

        pub fn new(id: i32, winning_numbers: Vec<i32>, numbers: Vec<i32>) -> ScratchCard {
            ScratchCard {
                id,
                winning_numbers,
                numbers,
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Point(pub i32);
    impl Sum for Point {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Point(0), Add::add)
        }
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point(self.0 + other.0)
        }
    }
    impl<'p> Add for &'p Point {
        type Output = Point;
        fn add(self, other: &Point) -> Point {
            Point(self.0 + other.0)
        }
    }

    pub fn parse_scratchcards(lines: &[String]) -> ScratchCardCollection {
        ScratchCardCollection {
            scratch_cards: lines.iter().map(|l| parse_scratchcard(l)).collect(),
        }
    }

    pub fn parse_scratchcard(line: &str) -> ScratchCard {
        let mut parts = line.split(":");
        let id = parse_id(parts.next().unwrap());
        let mut number_parts = parts.next().unwrap().split('|');
        ScratchCard {
            id,
            winning_numbers: parse_numbers(number_parts.next().unwrap()),
            numbers: parse_numbers(number_parts.next().unwrap()),
        }
    }

    fn parse_numbers(numbers_str: &str) -> Vec<i32> {
        numbers_str
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    }

    fn parse_id(card_name_str: &str) -> i32 {
        let num_regex = Regex::new(r"\d+").unwrap();
        let match_ = num_regex.find_iter(card_name_str).nth(0).unwrap();
        let digits = &card_name_str[match_.start()..match_.end()];
        digits.parse::<i32>().unwrap()
    }

    fn parse_id_2(card_name_str: &str) -> i32 {
        let bytes = card_name_str.as_bytes();
        let mut id_buf = [0_u8; 12];
        unsafe {
            asm!("nop");
        }

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::gambling::*;
    use common::file;

    #[test]
    fn can_find_sum_of_winnings_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let collection = parse_scratchcards(&lines);

        let winnings: Point = collection.winnings_part_1();

        assert_eq!(winnings.0, 13);
    }

    #[test]
    fn can_find_sum_of_winnings_for_example_file_part_2() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let collection = parse_scratchcards(&lines);

        let winnings = collection.winnings_part_2();

        assert_eq!(winnings.0, 30);
    }

    #[test]
    fn can_parse_scratch_card_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let scratch_card = parse_scratchcard(&line);

        assert_eq!(scratch_card.id(), 1);
        assert_eq!(scratch_card.points().0, 8);
    }

    #[test]
    fn can_find_winnings_for_scratchcard() {
        let scratch_card = ScratchCard::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );

        let points = scratch_card.points();

        assert_eq!(points.0, 8);
    }
}

mod stuff {
    #[derive(Debug)]
    pub struct RefEquality<'a, T>(pub &'a T);
    impl<'a, T> std::hash::Hash for RefEquality<'a, T> {
        fn hash<H>(&self, state: &mut H)
        where
            H: std::hash::Hasher,
        {
            (self.0 as *const T).hash(state)
        }
    }
    impl<'a, 'b, T> PartialEq<RefEquality<'b, T>> for RefEquality<'a, T> {
        fn eq(&self, other: &RefEquality<'b, T>) -> bool {
            self.0 as *const T == other.0 as *const T
        }
    }
    impl<'a, T> Eq for RefEquality<'a, T> {}
}
