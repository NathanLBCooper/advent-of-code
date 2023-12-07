use crate::day_07::{parse, solve};
use common::file;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let results = parse(&lines);

    println!("part 1: {}", solve(&results, false));
    println!("part 2: {}", solve(&results, true));
}

mod day_07 {
    use std::{cmp::Ordering, collections::HashMap};

    use common::core::IntoArr;

    #[derive(PartialEq, Debug)]
    pub enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    impl HandType {
        pub fn cmp(&self, other: &Self) -> Ordering {
            rank_hand_type(self).cmp(&rank_hand_type(other))
        }
    }

    fn rank_hand_type(hand_type: &HandType) -> u8 {
        match hand_type {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }

    #[derive(Debug)]
    pub struct Hand {
        pub cards: [char; 5],
        pub bid: i32,
    }

    impl Hand {
        pub fn get_type(&self, enable_wildcard: bool) -> HandType {
            const WILDCARD: char = 'J';

            let mut card_count = HashMap::<char, u8>::new();
            for card in self.cards.iter() {
                card_count.entry(*card).and_modify(|c| *c += 1).or_insert(1);
            }

            let mut count_of_counts = HashMap::<u8, u8>::new();
            for (c, count) in card_count.iter() {
                if enable_wildcard && *c == WILDCARD {
                    continue;
                }

                count_of_counts
                    .entry(*count)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }

            let mut hand_type = Hand::get_type_from_count_of_counts(&count_of_counts);

            if !enable_wildcard {
                return hand_type;
            }

            let wildcard_count = card_count.get(&WILDCARD).unwrap_or(&0).clone();
            for _ in 0..wildcard_count {
                hand_type = Hand::apply_wildcard(hand_type);
            }

            hand_type
        }

        fn get_type_from_count_of_counts(count_of_counts: &HashMap<u8, u8>) -> HandType {
            if count_of_counts.get(&5).is_some() {
                return HandType::FiveOfAKind;
            }

            if count_of_counts.get(&4).is_some() {
                return HandType::FourOfAKind;
            }

            let three_count = count_of_counts.get(&3);
            if three_count.is_some() {
                if count_of_counts.get(&2).is_some() {
                    return HandType::FullHouse;
                }

                return HandType::ThreeOfAKind;
            }

            let pair_count = count_of_counts.get(&2).unwrap_or(&0);
            if *pair_count == 2 {
                return HandType::TwoPair;
            }
            if *pair_count == 1 {
                return HandType::OnePair;
            }

            return HandType::HighCard;
        }

        fn apply_wildcard(hand_type: HandType) -> HandType {
            match hand_type {
                HandType::FiveOfAKind => HandType::FiveOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FullHouse => HandType::FourOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
            }
        }

        pub fn cmp(&self, other: &Self, enable_wildcard: bool) -> Ordering {
            let type_comparison = self
                .get_type(enable_wildcard)
                .cmp(&other.get_type(enable_wildcard));

            if type_comparison != Ordering::Equal {
                return type_comparison;
            }

            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let char_comparison = cmp_card(self_card, other_card, enable_wildcard);
                if char_comparison != Ordering::Equal {
                    return char_comparison;
                }
            }

            Ordering::Equal
        }
    }

    pub fn cmp_card(left: &char, right: &char, enable_wildcard: bool) -> Ordering {
        const ORDERED: [char; 13] = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];

        const ORDERED_FOR_WILDCARD: [char; 13] = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        let order = match enable_wildcard {
            true => &ORDERED_FOR_WILDCARD,
            false => &ORDERED,
        };

        let left_rank = order.iter().position(|c| *c == *left).unwrap();
        let right_rank = order.iter().position(|c| *c == *right).unwrap();

        return left_rank.cmp(&right_rank);
    }

    pub fn parse(lines: &Vec<String>) -> Vec<Hand> {
        lines.iter().map(|l| parse_hand(&l)).collect()
    }

    pub fn parse_hand(line: &str) -> Hand {
        let mut parts = line.split_whitespace();
        Hand {
            cards: to_cards(parts.next().unwrap()),
            bid: parts.next().unwrap().parse::<i32>().unwrap(),
        }
    }

    pub fn to_cards(s: &str) -> [char; 5] {
        s.chars().collect::<Vec<char>>().into_arr()
    }

    pub fn solve(hands: &Vec<Hand>, enable_wildcard: bool) -> i32 {
        let mut sorted_winner_last: Vec<&Hand> = hands.iter().collect::<Vec<&Hand>>();
        sorted_winner_last.sort_by(|l, r| (*l).cmp(*r, enable_wildcard));

        let mut total = 0;
        for (i, h) in sorted_winner_last.iter().enumerate() {
            total += (i as i32 + 1) * h.bid;
        }

        return total;
    }
}

#[cfg(test)]
mod tests {
    use crate::{day_07::*, tests::hand_builder::HandBuilder};
    use common::file;
    use std::cmp::Ordering;
    use test_case::test_case;

    #[test]
    fn can_solve_part_1_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let hands = parse(&lines);

        assert_eq!(solve(&hands, false), 6440);
    }

    #[ignore]
    #[test]
    fn can_solve_part_2_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();

        let hands = parse(&lines);

        assert_eq!(solve(&hands, true), 5905);
    }

    #[test]
    fn can_parse_hand() {
        let line = "32T3K 765";

        let hand = parse_hand(&line);

        assert_eq!(hand.bid, 765);
        assert_eq!(hand.cards, ['3', '2', 'T', '3', 'K']);
    }

    #[test]
    fn can_solve_part_1() {
        let hands = vec![
            Hand {
                cards: to_cards("32T3K"),
                bid: 765,
            },
            Hand {
                cards: to_cards("T55J5"),
                bid: 684,
            },
            Hand {
                cards: to_cards("KK677"),
                bid: 28,
            },
            Hand {
                cards: to_cards("KTJJT"),
                bid: 220,
            },
            Hand {
                cards: to_cards("QQQJA"),
                bid: 483,
            },
        ];

        assert_eq!(solve(&hands, false), 6440);
    }

    #[test]
    fn can_order_hands_by_type() {
        // Not happy with this builder and test case is probably better for this kind of problem anyway
        let five_of_a_kind = HandBuilder::new().with_type(HandType::FiveOfAKind).build();
        let four_of_a_kind = HandBuilder::new().with_type(HandType::FourOfAKind).build();
        let full_house = HandBuilder::new().with_type(HandType::FullHouse).build();
        let three_of_a_kind = HandBuilder::new().with_type(HandType::ThreeOfAKind).build();
        let two_pair = HandBuilder::new().with_type(HandType::TwoPair).build();
        let one_pair = HandBuilder::new().with_type(HandType::OnePair).build();
        let high = HandBuilder::new().with_type(HandType::HighCard).build();

        let unordered_hands = [
            &high,
            &three_of_a_kind,
            &full_house,
            &two_pair,
            &five_of_a_kind,
            &four_of_a_kind,
            &one_pair,
        ];

        let mut sorted = unordered_hands.iter().map(|h| *h).collect::<Vec<&Hand>>();
        sorted.sort_by(|l, r| r.cmp(l, false));

        assert_ref_eq(sorted[0], &five_of_a_kind);
        assert_ref_eq(sorted[1], &four_of_a_kind);
        assert_ref_eq(sorted[2], &full_house);
        assert_ref_eq(sorted[3], &three_of_a_kind);
        assert_ref_eq(sorted[4], &two_pair);
        assert_ref_eq(sorted[5], &one_pair);
        assert_ref_eq(sorted[6], &high);
    }

    #[test_case("222JJ", HandType::FiveOfAKind; "five of a kind")]
    #[test_case("222J3", HandType::FourOfAKind; "four of a kind")]
    #[test_case("J2233", HandType::FullHouse; "gives full house")]
    #[test_case("K2JJ3", HandType::ThreeOfAKind; "gives three of a kind")]
    #[test_case("22J34", HandType::ThreeOfAKind; "gives three of a kind, not a full house")] // ie don't double count the J
    #[test_case("J2345", HandType::OnePair; "gives pair")]
    fn can_find_type_with_wildcard(cards: &str, hand_type: HandType) {
        assert_eq!(
            HandBuilder::new()
                .with_cards(to_cards(cards))
                .build()
                .get_type(true),
            hand_type
        );
    }

    #[test_case("2KKKK", "33332", false)]
    #[test_case("22228", "22229", false)]
    #[test_case("KAAAA", "AKKKK", false)]
    #[test_case("2AAAA", "JKKKK", false)]
    #[test_case("JAAA2", "2KKKJ", true; "J is weak when wildcard")]
    fn can_order_hands_of_the_same_type_based_on_leading_numbers(
        lower: &str,
        higher: &str,
        enable_wildcard: bool,
    ) {
        let lower_hand = HandBuilder::new().with_cards(to_cards(lower)).build();
        let higher_hand = HandBuilder::new().with_cards(to_cards(higher)).build();

        assert_eq!(
            higher_hand.cmp(&lower_hand, enable_wildcard),
            Ordering::Greater
        );
    }

    fn assert_ref_eq(l: &Hand, r: &Hand) {
        // I'm assuming this is awkward because one shouldn't do it?
        assert_eq!(l as *const Hand, r as *const Hand, "{:?}{:?}", l, r);
    }

    // bit over the top to make a builder. Especially since mut hand is an option. But I do this a lot in C' i wanted to see how it is
    mod hand_builder {
        use crate::day_07::{Hand, HandType};

        pub struct HandBuilder {
            pub cards: [char; 5],
            pub bid: i32,
        }

        impl HandBuilder {
            pub fn new() -> HandBuilder {
                HandBuilder {
                    cards: ['1', '2', '3', '4', '5'],
                    bid: 10,
                }
            }

            pub fn with_cards(mut self, cards: [char; 5]) -> Self {
                self.cards = cards;
                self
            }

            pub fn with_type(mut self, hand_type: HandType) -> Self {
                let a = '6';
                let b = '7';
                match hand_type {
                    HandType::FiveOfAKind => self.cards = [a, a, a, a, a],
                    HandType::FourOfAKind => self.cards = [a, a, a, a, self.cards[4]],
                    HandType::FullHouse => self.cards = [a, a, a, b, b],
                    HandType::ThreeOfAKind => self.cards = [a, a, a, self.cards[3], self.cards[4]],
                    HandType::TwoPair => self.cards = [a, a, b, b, self.cards[4]],
                    HandType::OnePair => {
                        self.cards = [a, a, self.cards[2], self.cards[3], self.cards[4]]
                    }
                    HandType::HighCard => (),
                };

                self
            }

            pub fn build(&self) -> Hand {
                Hand {
                    cards: self.cards,
                    bid: self.bid,
                }
            }
        }
    }
}
