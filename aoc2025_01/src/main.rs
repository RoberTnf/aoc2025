use std::io::BufRead;

struct Dial {
    pub position: i32,
    pub max_position: i32,
}

impl Dial {
    pub fn new(position: i32, max_position: i32) -> Dial {
        Dial {
            position,
            max_position,
        }
    }

    pub fn rotate(&mut self, steps: i32) {
        self.position = (self.position + steps) % self.max_position;
    }

    pub fn rotate_pass_through_zero(&mut self, steps: i32) -> i32 {
        // could be much simplified to not use a loop and instead use euclidean division,
        // but I didn't bother, it is fast enough
        let start_position = self.position;
        self.position = self.position + steps;
        let mut counter = 0;

        if self.position == 0 {
            // rotate to exactly 0
            counter += 1;
        } else if self.position < 0 {
            // rotate to negative
            if start_position == 0 {
                counter -= 1; // if we start at 0, the first time it doesn't count as it was counted before
            }
            while self.position < 0 {
                self.position += self.max_position;
                counter += 1;
            }
            if self.position == 0 {
                // if we end at 0. For example -100 means you go to zero twice
                counter += 1;
            }
        } else if self.position >= self.max_position {
            // rotate to positive
            while self.position >= self.max_position {
                self.position -= self.max_position;
                counter += 1;
            }
            // no edge cases here
        }

        counter
    }
}

fn process_data(fname: &str) -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();
    let file = std::fs::File::open(fname).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let direction = line.chars().next().unwrap();
        let mut steps = line[1..].parse::<i32>().unwrap();

        if direction == 'L' {
            steps = -steps;
        }
        data.push(steps);
    }
    data
}

fn solve_first(data: Vec<i32>) {
    let mut dial = Dial::new(50, 100);
    let mut counter = 0;
    for step in data {
        dial.rotate(step);
        if dial.position == 0 {
            counter += 1;
        }
    }
    println!("first solution: {}", counter);
}

fn solve_second(data: Vec<i32>) {
    let mut dial = Dial::new(50, 100);
    let mut counter = 0;
    for step in data {
        counter += dial.rotate_pass_through_zero(step);
    }
    println!("second solution: {}", counter);
}

fn main() {
    let data = process_data("data/data.txt");
    // let test_data = process_data("data/test.txt");
    //solve_first(test_data.clone());
    //solve_second(test_data);
    solve_first(data.clone());
    solve_second(data);
}
