use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::cell::RefCell;
#[derive(Debug)]
struct Robot {
    id: u8,
    holding: RefCell<Vec<u8>>,
    high: u8,
    low: u8,
    high_output: bool,
    low_output: bool,
}

impl Robot {
    fn push(&self, num: u8) {
        self.holding.borrow_mut().push(num);
    }
    fn len(&self) -> usize {
        self.holding.borrow().len()
    }
}

fn main() {
    let mut robots: HashMap<u8, Robot> = HashMap::new();
    let mut input = String::new();
    let mut file = File::open("../resources/day10.txt").unwrap();
    match file.read_to_string(&mut input) {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(_) => (),
    }
    let mut values: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.contains("value") {
            values.push(line);
        } else if line.contains("low") {
            let split: Vec<&str> = line.split(" ").collect();
            let id = split[1].parse().unwrap();
            let high_output = split[10].contains("out");
            let low_output = split[5].contains("out");
            let robot = Robot {
                id: id,
                holding: RefCell::new(Vec::new()),
                high: split[11].parse().unwrap(),
                low: split[6].parse().unwrap(),
                high_output: high_output,
                low_output: low_output,
            };
            robots.insert(id, robot);
        }
    }
    let mut product = 1;
    for value in values {
        let split: Vec<&str> = value.split(" ").collect();
        let robot = robots.get(&split[5].parse().unwrap()).unwrap();
        robot.push(split[1].parse().unwrap());
        if robot.len() == 2 {
            pass_off(robot, &robots, &mut product);
        }
    }
    println!("Product of outputs 0, 1, and 2: {}", product);
}

fn pass_off(robot: &Robot, robots: &HashMap<u8, Robot>, product: &mut u16) {
    let high_num: u8;
    let low_num: u8;
    {
        let mut numbers = robot.holding.borrow_mut();
        if numbers[0] > numbers[1] {
            high_num = numbers[0];
            low_num = numbers[1];
        } else {
            high_num = numbers[1];
            low_num = numbers[0];
        }
        numbers.clear();
    }
    if high_num == 61 && low_num == 17 {
        println!("Bot {} compares microchip 61 and 17", robot.id);
    }
    if !robot.high_output {
        match robots.get(&robot.high) {
            Some(high) => {
                high.push(high_num);
                if high.len() == 2 {
                    pass_off(high, robots, product);
                }
            }
            None => println!("Error"),
        }
    } else {
        if robot.high == 1 || robot.high == 2 || robot.high == 0 {
            *product *= high_num as u16;
        }
    }
    if !robot.low_output {
        match robots.get(&robot.low) {
            Some(low) => {
                low.push(low_num);
                if low.len() == 2 {
                    pass_off(low, robots, product);
                }
            }
            None => println!("Error"),
        }
    } else {
        if robot.low == 1 || robot.low == 2 || robot.low == 0 {
            *product *= low_num as u16;
        }
    }
}