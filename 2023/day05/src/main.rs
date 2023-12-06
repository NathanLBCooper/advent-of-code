use common::file;

use crate::almanac::parse;

fn main() {
    let file = String::from("./input.txt");
    let lines = file::read_lines(&file).unwrap();
    let lines_ref: Vec<&str> = lines.iter().map(String::as_ref).collect();

    let almanac = parse(&lines_ref);

    println!("Part 1: {}", almanac.calculate_min_location());
    println!("Part 2: {}", almanac.calculate_min_location_part2())
}

mod almanac {
    use std::{cmp, slice::Iter};

    use common::{core::into_arr, parsing};

    pub struct Almanac {
        pub seed_ids: Vec<i64>,
        pub seed_ranges: Vec<Range>,
        pub seed_to_soil_map: Map,
        pub soil_to_fertilizer_map: Map,
        pub fertilizer_to_water_map: Map,
        pub water_to_light_map: Map,
        pub light_to_temperature_map: Map,
        pub temperature_to_humidity_map: Map,
        pub humidity_to_location_map: Map,
    }

    impl Almanac {
        pub fn map(&self, seed_id: i64) -> i64 {
            let soil = self.seed_to_soil_map.map(seed_id);
            let fertilizer = self.soil_to_fertilizer_map.map(soil);
            let water = self.fertilizer_to_water_map.map(fertilizer);
            let light = self.water_to_light_map.map(water);
            let temp = self.light_to_temperature_map.map(light);
            let humidity = self.temperature_to_humidity_map.map(temp);
            let location = self.humidity_to_location_map.map(humidity);

            location
        }

        pub fn calculate_min_location(&self) -> i64 {
            let locations = self.seed_ids.iter().map(|s| self.map(*s));

            locations.min().unwrap()
        }

        pub fn calculate_min_location_part2(&self) -> i64 {
            let seeds = self.seed_ranges.iter().map(|r| r.expand());

            let foo: Vec<i64> = seeds.flatten().collect();

            let locations = foo.iter().map(|s| self.map(*s));

            locations.min().unwrap()
        }

        pub fn calculate_min_location_part2_2(&self) -> i64 {
            // todo, here is the plan:
            // replace range with std range. Range Map with some { std range, number_to_add }
            // Then flatten the chain of "vectors" into a collection of vectors from seed to location
            // then... stuff

            let mut ranges = flatten(
                &self.seed_to_soil_map.ranges,
                &self.soil_to_fertilizer_map.ranges,
            );

            ranges.sort_by(|l, r| r.destination.cmp(&l.destination));

            for range in ranges {
                for seeds in &self.seed_ranges {
                    if seeds.overlaps(Range {
                        start: range.source,
                        length: range.length,
                    }) {
                        let foo = cmp::max(seeds.start, range.source); // todo get from range, also, what's foo?
                    }
                }
            }

            panic!();
        }
    }

    pub fn flatten(first: &Vec<RangeMap>, second: &Vec<RangeMap>) -> Vec<RangeMap> {
        todo!(); // todo, whatever, brute force returned correct answer before I implemented this
    }

    pub struct Map {
        pub ranges: Vec<RangeMap>,
    }

    impl Map {
        pub fn map(&self, source_id: i64) -> i64 {
            let matching_range = self.ranges.iter().find(|r| r.contains(source_id));

            match matching_range {
                Some(r) => r.transform(source_id),
                None => source_id,
            }
        }
    }

    pub struct Range {
        pub start: i64,
        pub length: i64,
    }

    impl Range {
        pub fn expand(&self) -> Vec<i64> {
            return (self.start..self.upper()).collect();
        }

        pub fn overlaps(&self, other: Range) -> bool {
            self.upper() >= other.start && self.start <= other.upper()
        }

        pub fn upper(&self) -> i64 {
            self.start + self.length
        }
    }

    pub struct RangeMap {
        pub source: i64,
        pub destination: i64,
        pub length: i64,
    }

    impl RangeMap {
        // horrible duplicate of range
        pub fn contains(&self, source_id: i64) -> bool {
            let upper = self.source + self.length;
            self.source <= source_id && source_id < upper
        }

        pub fn transform(&self, n_source: i64) -> i64 {
            let offset = self.destination - self.source;
            n_source + offset
        }
    }

    pub fn parse(lines: &[&str]) -> Almanac {
        let mut iter = lines.iter();

        let line_one = iter.next().unwrap();
        let seed_ids = parse_seed_ids(line_one);

        let seed_ranges: Vec<Range> = seed_ids
            .chunks(2)
            .map(|c| Range {
                start: c[0],
                length: c[1],
            })
            .collect();

        let _whitespace = iter.next();

        Almanac {
            seed_ids,
            seed_ranges,
            seed_to_soil_map: parse_next_map(&mut iter),
            soil_to_fertilizer_map: parse_next_map(&mut iter),
            fertilizer_to_water_map: parse_next_map(&mut iter),
            water_to_light_map: parse_next_map(&mut iter),
            light_to_temperature_map: parse_next_map(&mut iter),
            temperature_to_humidity_map: parse_next_map(&mut iter),
            humidity_to_location_map: parse_next_map(&mut iter),
        }
    }

    fn parse_next_map(iter: &mut Iter<'_, &str>) -> Map {
        let _title = iter.next();
        parse_maps(parse_number_lines_until_end_or_empty_line(iter))
    }

    fn parse_seed_ids(line: &str) -> Vec<i64> {
        let mut parts = line.split(":");
        let _section_header = parts.next();
        let seed_id_line = parts.next().unwrap();
        let seed_ids = parsing::parse_numbers::<i64>(seed_id_line);

        seed_ids.iter().map(|id| id.clone()).collect()
    }

    fn parse_maps(numbers: Vec<Vec<i64>>) -> Map {
        let ranges: Vec<RangeMap> = numbers.into_iter().map(|l| parse_range_map(l)).collect();

        Map { ranges }
    }

    fn parse_range_map(numbers: Vec<i64>) -> RangeMap {
        let number_arr = into_arr::<i64, 3>(numbers);
        RangeMap {
            source: number_arr[1],
            destination: number_arr[0],
            length: number_arr[2],
        }
    }

    fn parse_number_lines_until_end_or_empty_line(iter: &mut Iter<'_, &str>) -> Vec<Vec<i64>> {
        let mut result_lines: Vec<Vec<i64>> = Vec::new();
        loop {
            match iter.next() {
                Some(line) => match line.is_empty() {
                    true => break,
                    false => {
                        let nums = parsing::parse_numbers(line);
                        result_lines.push(nums)
                    }
                },
                None => break,
            }
        }

        return result_lines;
    }
}

#[cfg(test)]
mod tests {
    use crate::almanac::*;
    use common::file;

    #[test]
    fn can_solve_part_1_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();
        let lines_ref: Vec<&str> = lines.iter().map(String::as_ref).collect();

        let almanac = parse(&lines_ref);

        assert_eq!(almanac.calculate_min_location(), 35);
    }

    #[test]
    fn can_solve_part_2_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();
        let lines_ref: Vec<&str> = lines.iter().map(String::as_ref).collect();

        let almanac = parse(&lines_ref);

        assert_eq!(almanac.calculate_min_location_part2(), 46);
    }

    #[test]
    fn can_map_from_seed_to_location_for_example_file() {
        let file = String::from("./example.txt");
        let lines = file::read_lines(&file).unwrap();
        let lines_ref: Vec<&str> = lines.iter().map(String::as_ref).collect();

        let almanac = parse(&lines_ref);

        assert_eq!(almanac.map(79), 82);
        assert_eq!(almanac.map(14), 43);
    }

    #[test]
    fn map_works() {
        let map = Map {
            ranges: vec![
                RangeMap {
                    source: 10,
                    destination: 100,
                    length: 2,
                },
                RangeMap {
                    source: 20,
                    destination: 200,
                    length: 4,
                },
            ],
        };

        assert_eq!(map.map(0), 0);
        assert_eq!(map.map(9), 9);

        assert_eq!(map.map(10), 100);
        assert_eq!(map.map(11), 101);
        assert_eq!(map.map(12), 12);

        assert_eq!(map.map(20), 200);
    }

    #[test]
    fn range_map_works() {
        let range = RangeMap {
            source: 10,
            destination: 100,
            length: 2,
        };

        assert!(!range.contains(9));
        assert!(range.contains(10));
        assert!(range.contains(11));
        assert!(!range.contains(12));
    }

    #[test]
    fn can_parse() {
        let lines = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            //etc
        ];

        let almanac = parse(&lines);

        {
            assert_eq!(almanac.seed_ids.iter().count(), 4);
            let mut iter = almanac.seed_ids.iter();
            assert_eq!(*iter.next().unwrap(), 79);
            assert_eq!(*iter.next().unwrap(), 14);
            assert_eq!(*iter.next().unwrap(), 55);
            assert_eq!(*iter.next().unwrap(), 13);
        }

        {
            assert_eq!(almanac.seed_to_soil_map.ranges.iter().count(), 2);
            let mut iter = almanac.seed_to_soil_map.ranges.iter();
            let range_one = iter.next().unwrap();
            assert_eq!(range_one.source, 98);
            assert_eq!(range_one.destination, 50);
            assert_eq!(range_one.length, 2);
            let range_two = iter.next().unwrap();
            assert_eq!(range_two.source, 50);
            assert_eq!(range_two.destination, 52);
            assert_eq!(range_two.length, 48);
        }

        // etc
    }
}
