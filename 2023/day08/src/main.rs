use std::{cell::RefCell, rc::Rc};

use common::file;

use crate::wasteland_map::parse;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();

    let map = parse(&lines);

    // this is too slow to find the answer (if it's even correct at all). Something smarter is in order. I've spent a long time tangling with Rc and RefCell and I'm done for now
    println!("part 1: {}", map.distance_to_end());
    println!("part 2: {}", map.part_2());
}

mod wasteland_map {
    use common::core::IntoArr;
    use num::Integer;

    use crate::linked_nodes::{self, new_rc, Node};
    use std::{cell::RefCell, collections::HashMap, rc::Rc, slice::Iter};

    #[derive(Debug)]
    pub enum Direction {
        Left,
        Right,
    }

    #[derive(Debug)]
    pub struct Element {
        pub name: String,
    }

    impl Element {
        pub fn is_end(&self) -> bool {
            self.name == "ZZZ"
        }

        pub fn is_end_part2(&self) -> bool {
            self.name.chars().nth(2).unwrap() == 'Z'
        }
    }

    pub struct WastelandMap {
        pub instructions: Vec<Direction>,
        pub start: Rc<RefCell<linked_nodes::Node<Element>>>, // todo, I don't think I need to mutate this already build graph. but not sure how to get non-mutable type
        pub part_2_starts: Vec<Rc<RefCell<Node<Element>>>>,
    }

    impl WastelandMap {
        pub fn distance_to_end(&self) -> i64 {
            let mut node = self.start.clone();
            let mut distance: i64 = 1;
            loop {
                for instruction in self.instructions.iter() {
                    match instruction {
                        Direction::Left => {
                            let left = node.borrow().left.as_ref().unwrap().clone();
                            node = left;
                        }
                        Direction::Right => {
                            let right = node.borrow().right.as_ref().unwrap().clone();
                            node = right;
                        }
                    }

                    let current_node = &node.borrow();
                    if current_node.value.is_end() {
                        return distance;
                    }

                    distance += 1;
                }

                // if distance > 100 {
                //     panic!("loop");
                // }
            }
        }

        pub fn distance_to_end_part2(
            start: &Rc<RefCell<linked_nodes::Node<Element>>>,
            instructions: &Vec<Direction>,
        ) -> i64 {
            let mut node = start.clone();
            let mut distance: i64 = 1;
            loop {
                for instruction in instructions.iter() {
                    match instruction {
                        Direction::Left => {
                            let left = node.borrow().left.as_ref().unwrap().clone();
                            node = left;
                        }
                        Direction::Right => {
                            let right = node.borrow().right.as_ref().unwrap().clone();
                            node = right;
                        }
                    }

                    let current_node = &node.borrow();
                    if current_node.value.is_end_part2() {
                        return distance;
                    }

                    distance += 1;
                }

                // if distance > 100 {
                //     panic!("loop");
                // }
            }
        }

        pub fn part_2(&self) -> i64 {
            let cycle_lengths: Vec<i64> = self
                .part_2_starts
                .iter()
                .map(|s| WastelandMap::distance_to_end_part2(s, &self.instructions))
                .collect();

            // smallest possible answer, if cycles are neat and circular is lcm, try that
            let result = cycle_lengths
                .iter()
                .fold(1, |acc, &x| num::integer::lcm(acc, x.abs() as i64) as i64);

            // yup. that's it

            return result;
        }
    }

    pub fn parse(lines: &Vec<String>) -> WastelandMap {
        let mut iter = lines.iter();

        let instructions = parse_instructions(iter.next().unwrap());
        let _blank_line = iter.next();

        let (start, part_2_starts) = parse_nodes(&mut iter);

        // println!("{:?}", instructions);

        WastelandMap {
            instructions,
            start,
            part_2_starts,
        }
    }

    pub fn parse_instructions(line: &str) -> Vec<Direction> {
        line.chars()
            .filter_map(|c| match c {
                'L' => Some(Direction::Left),
                'R' => Some(Direction::Right),
                _ => None,
            })
            .collect()
    }

    pub fn parse_nodes(
        lines: &mut Iter<String>,
    ) -> (Rc<RefCell<Node<Element>>>, Vec<Rc<RefCell<Node<Element>>>>) {
        let lines_arr: Vec<[char; 16]> = lines
            .map(|l| l.chars().collect::<Vec<char>>().into_arr::<16>())
            .collect();

        let mut start: Option<Rc<RefCell<Node<Element>>>> = None;
        let mut part_2_starts: Vec<Rc<RefCell<Node<Element>>>> = Vec::new();
        let mut nodes = HashMap::<String, Rc<RefCell<Node<Element>>>>::new();
        for l in lines_arr.iter() {
            let value = Element {
                name: l[0..3].iter().collect(),
            };

            let key = value.name.clone();
            let node: Rc<RefCell<Node<Element>>> = new_rc(Node::<Element>::new(value));

            if key == "AAA" {
                start = Some(node.clone());
            }

            if key.chars().nth(2).unwrap() == 'A' {
                part_2_starts.push(node.clone())
            }

            nodes.insert(key, node);
        }

        // this is a very helpful loop for creating memory leaks via closed cycles of referencing Rcs
        for l in lines_arr.iter() {
            let key = l[0..3].iter().collect::<String>();
            let left_key = l[7..10].iter().collect::<String>();
            let right_key = l[12..15].iter().collect::<String>();

            let node = nodes.get(&key).unwrap();
            let left_node = nodes.get(&left_key).unwrap().clone();
            let right_node = nodes.get(&right_key).unwrap().clone();

            node.borrow_mut().attach_left(left_node);
            node.borrow_mut().attach_right(right_node);

            // println!("{:?}", &node);
            // println!("{:?}", &l);
            // println!();
        }

        (start.unwrap(), part_2_starts)
    }
}

mod linked_nodes {
    use std::{cell::RefCell, rc::Rc};

    pub struct Node<T> {
        pub value: T,
        pub left: Option<Rc<RefCell<Node<T>>>>,
        pub right: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("value", &self.value)
                .field("left", &self.left.as_ref().unwrap().borrow().value) // todo panic for None
                .field("right", &self.right.as_ref().unwrap().borrow().value)
                .finish()
        }
    }

    impl<'a, T> Node<T> {
        pub fn new(value: T) -> Node<T> {
            Node::<T> {
                value: value,
                left: None,
                right: None,
            }
        }

        pub fn attach_left(&mut self, other: Rc<RefCell<Node<T>>>) {
            self.left = Some(other.clone());
        }

        pub fn attach_right(&mut self, other: Rc<RefCell<Node<T>>>) {
            self.right = Some(other.clone());
        }
    }

    pub fn new_rc<T>(node: Node<T>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(node))
    }
}

#[cfg(test)]
mod wasteland_map_tests {
    use crate::{
        linked_nodes::{new_rc, Node},
        wasteland_map::{Direction, Element, WastelandMap},
    };
    use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc};

    #[test]
    fn example_works() {
        let mut a: Rc<RefCell<Node<Element>>> = new_rc(Node::<Element>::new(Element {
            name: String::from("AAA"),
        }));
        let mut b = new_rc(Node::<Element>::new(Element {
            name: String::from("BBB"),
        }));
        let mut c = new_rc(Node::<Element>::new(Element {
            name: String::from("CCC"),
        }));
        let mut d = new_rc(Node::<Element>::new(Element {
            name: String::from("DD"),
        }));
        let mut e = new_rc(Node::<Element>::new(Element {
            name: String::from("EEE"),
        }));
        let mut g = new_rc(Node::<Element>::new(Element {
            name: String::from("GGG"),
        }));
        let mut z = new_rc(Node::<Element>::new(Element {
            name: String::from("ZZZ"),
        }));

        a.borrow_mut().as_ref().borrow_mut().attach_left(b.clone());
        a.borrow_mut().as_ref().borrow_mut().attach_right(c.clone());

        b.borrow_mut().as_ref().borrow_mut().attach_left(d.clone());
        b.borrow_mut().as_ref().borrow_mut().attach_right(e.clone());

        c.borrow_mut().as_ref().borrow_mut().attach_left(z.clone());
        c.borrow_mut().as_ref().borrow_mut().attach_right(g.clone());

        let map = WastelandMap {
            instructions: vec![Direction::Right, Direction::Left],
            start: a.clone(),
            part_2_starts: Vec::new(),
        };

        let distance = map.distance_to_end();

        assert_eq!(distance, 2);
    }

    #[test]
    fn other_example_works() {
        let mut a: Rc<RefCell<Node<Element>>> = new_rc(Node::<Element>::new(Element {
            name: String::from("AAA"),
        }));
        let mut b = new_rc(Node::<Element>::new(Element {
            name: String::from("BBB"),
        }));
        let mut z = new_rc(Node::<Element>::new(Element {
            name: String::from("ZZZ"),
        }));

        a.borrow_mut().as_ref().borrow_mut().attach_left(b.clone());
        a.borrow_mut().as_ref().borrow_mut().attach_right(b.clone());

        b.borrow_mut().as_ref().borrow_mut().attach_left(a.clone());
        b.borrow_mut().as_ref().borrow_mut().attach_right(z.clone());

        let map = WastelandMap {
            instructions: vec![Direction::Left, Direction::Left, Direction::Right],
            start: a.clone(),
            part_2_starts: Vec::new(),
        };

        let distance = map.distance_to_end();

        assert_eq!(distance, 6);
    }
}

#[cfg(test)]
mod doubly_linked_list_tests {
    use crate::linked_nodes::{new_rc, Node};
    use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

    #[test]
    fn linked_nodes_works() {
        let mut one: Rc<RefCell<Node<u8>>> = new_rc(Node::<u8>::new(1));
        let mut two: Rc<RefCell<Node<u8>>> = new_rc(Node::<u8>::new(2));
        let mut three = new_rc(Node::<u8>::new(3));

        // left to right 1,2,3
        one.borrow_mut()
            .as_ref()
            .borrow_mut()
            .attach_right(two.clone());
        two.borrow_mut()
            .as_ref()
            .borrow_mut()
            .attach_right(three.clone());

        // left goes back to 1
        two.borrow_mut()
            .as_ref()
            .borrow_mut()
            .attach_left(one.clone());
        three
            .borrow_mut()
            .as_ref()
            .borrow_mut()
            .attach_left(one.clone());

        let twice_right_from_one = one
            .borrow()
            .right
            .as_deref()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap();
        assert_eq!(twice_right_from_one.borrow().value, 3);

        let left_from_three = three.borrow().left.clone().unwrap();
        assert_eq!(left_from_three.borrow().value, 1);

        let left_from_two = two.borrow().left.clone().unwrap();
        assert_eq!(left_from_two.borrow().value, 1);
    }
}
