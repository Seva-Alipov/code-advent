use std::fs;

#[derive(Clone, Copy)]
pub struct Range {
    low: usize,
    high: usize
}
impl Range {
    pub fn in_range(&self, val: usize) -> bool { //Used in part 1
        self.low <= val && val <= self.high
    }
    pub fn has_range(&self) -> usize {
        self.high - self.low
    }
}

pub fn parse_ranges(ranges_str: &str) -> Vec<Range> {
    let bytes = ranges_str.as_bytes();
    let mut ranges = Vec::new();

    let mut startnum = 0;
    let mut endnum = 0;
    let mut current_from: usize = 0;
    let mut current_to: usize;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'\n' => {
                current_to = ranges_str[startnum..endnum].parse().unwrap();
                ranges.push(Range { low: current_from, high: current_to });
                startnum = i+1;
                endnum = i+1;
            }
            b'-' => {
                current_from = ranges_str[startnum..endnum].parse().unwrap();
                startnum = i+1;
                endnum = i+1;
            }
            _ => {endnum += 1;}
        }
    }

    ranges
}

pub fn count_fresh(ranges: &Vec<Range>, values: &Vec<usize>) -> usize { //PART 1
    let mut freshes = 0;
    for value in values {
        for range in ranges {
            if range.in_range(*value) {
                freshes += 1;
                break;
            }
        }
    }

    freshes
}

pub fn fresh_indices(ranges: &Vec<Range>) -> usize { //PART 2 WIP
    let mut ranges_cpy = ranges.clone();
    let mut ids: usize = 0;

    for i in 0..ranges_cpy.len() {
        let from = ranges_cpy[i].low;
        let to = ranges_cpy[i].high;

        for j in i+1..ranges_cpy.len() {
            if ranges_cpy[j].low == 0 && ranges_cpy[j].high == 0 {continue;}

            if ranges_cpy[j].in_range(from) && ranges_cpy[j].in_range(to) {
                ranges_cpy[i].low = 0;
                ranges_cpy[i].high = 0;
                continue;
            }

            if ranges_cpy[i].in_range(ranges_cpy[j].low) && ranges_cpy[i].in_range(ranges_cpy[j].high) {
                ranges_cpy[j].low = 0;
                ranges_cpy[j].high = 0;
                continue;
            }

            if ranges_cpy[j].in_range(from) {
                ranges_cpy[j].high = from-1;
                if ranges_cpy[j].low > ranges_cpy[j].high {
                    ranges_cpy[j].low = 0;
                    ranges_cpy[j].high = 0;
                }
            }
            if ranges_cpy[j].in_range(to) {
                ranges_cpy[j].low = to+1;
                if ranges_cpy[j].low > ranges_cpy[j].high {
                    ranges_cpy[j].high = 0;
                    ranges_cpy[j].low = 0;
                }

            }
        }
    }

    for range in ranges_cpy {
        ids += match range.has_range() {
            0 => {
                if range.low == 0 && range.high == 0 {0}
                else {1}
            }
            _ => range.has_range() + 1
        }
    }

    ids
}

fn main() {
    let input = fs::read_to_string("problem.txt").expect("Could not read file.");

    let range_str: &str;
    let ids_str: &str;

    if let Some((first, second)) = input.split_once("\n\n") {
        range_str = first;
        ids_str = second;
    }
    else {panic!("Couldn't split string.")}

    let ranges: Vec<Range> = parse_ranges(range_str);
    let values: Vec<usize> = ids_str.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let freshes = count_fresh(&ranges, &values);
    let fresh_indices = fresh_indices(&ranges);

    println!("There are {freshes} fresh items left.");
    println!("There are {fresh_indices} indices that are fresh.");
}
