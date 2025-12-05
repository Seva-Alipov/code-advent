use std::fs;

pub struct Dial {
    zeroes_so_far: usize,
    current_value: isize,
}

impl Dial {
    pub fn new() -> Self{
        Dial{zeroes_so_far: 0, current_value: 50}
    }
    pub fn left(&mut self, turn_by: isize) {
        self.current_value -= turn_by;

        if self.current_value == 0 {
            self.zeroes_so_far += 1;
            return;
        }

        while self.current_value < 0 {
            self.current_value += 100;
        }
        self.current_value %= 100;

        if self.current_value == 0 {self.zeroes_so_far += 1;}
    }
    pub fn right(&mut self, turn_by: isize) {
        self.current_value += turn_by;

        if self.current_value == 0 {
            self.zeroes_so_far += 1;
            return;
        }

        while self.current_value < 0 {
            self.current_value += 100;
        }
        self.current_value %= 100;

        if self.current_value == 0 {self.zeroes_so_far += 1;}
    }

    pub fn left_alt(&mut self, turn_by: isize) {
        self.current_value -= turn_by;

        if self.current_value == 0 {
            self.zeroes_so_far += 1;
            return;
        }

        while self.current_value < 0 {
            self.current_value += 100;
            self.zeroes_so_far += 1;
        }
        while self.current_value > 99 {
            self.current_value -= 100;
            self.zeroes_so_far += 1;
        }

        if self.current_value == 0 {self.zeroes_so_far += 1;}
    }
    pub fn right_alt(&mut self, turn_by: isize) {
        self.current_value += turn_by;

        if self.current_value == 0 {
            self.zeroes_so_far += 1;
            return;
        }

        while self.current_value < 0 {
            self.current_value += 100;
            self.zeroes_so_far += 1;
        }
        while self.current_value > 99 {
            self.current_value -= 100;
            self.zeroes_so_far += 1;
        }

        if self.current_value == 0 {self.zeroes_so_far += 1;}
    }

    pub fn get_pwd(&self) -> usize {
        self.zeroes_so_far
    }
}

fn main() {

    let mut dial = Dial::new();
    let mut dial_2 = Dial::new();

    let instructions = fs::read_to_string(String::from("./instructions.txt")).expect("Could not read file. Make sure there exists an instruc");
    for instruction in instructions.split_whitespace() {
        match instruction.chars().nth(0).unwrap() {
            'L' | 'l' => {
                dial.left(instruction[1..].parse().expect("Could not parse usize"));
                dial_2.left_alt(instruction[1..].parse().expect("Could not parse usize"));
            }
            'R' | 'r' => {
                dial.right(instruction[1..].parse().expect("Could not parse usize"));
                dial_2.right(instruction[1..].parse().expect("Could not parse usize"));
            }
            _ => panic!("Could not parse instruction.")
        }
    }

    let pwd = dial.get_pwd();
    let pwd_alt = dial_2.get_pwd();
    println!("The Password is: {pwd}");
    println!("The other paassword is: {pwd_alt}");
}
