use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("../resources/day12.txt").unwrap();
    match file.read_to_string(&mut input) {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(_) => (),
    }
    let mut registers: HashMap<char, i32> = HashMap::with_capacity(4);
    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    let assembly: Vec<&str> = input.lines().collect();
    let mut i = 0;
    while i < assembly.len() {
        let line: Vec<&str> = assembly[i].split(" ").collect();
        let cmd = line[0];
        if cmd == "inc" {
            let reg = line[1].chars().nth(0).unwrap();
            let val = registers.entry(reg).or_insert(0);
            *val += 1;
            i += 1;
        } else if cmd == "dec" {
            let reg = line[1].chars().nth(0).unwrap();
            let val = registers.entry(reg).or_insert(0);
            *val -= 1;
            i += 1;
        } else if cmd == "cpy" {
            let op = line[1].parse::<i32>();
            match op {
                Ok(num) => {
                    let reg = line[2].chars().nth(0).unwrap();
                    let val = registers.entry(reg).or_insert(0);
                    *val = num;
                }
                Err(..) => {
                    let val1 = registers.get(&line[1].chars().nth(0).unwrap()).unwrap().clone();
                    let reg2 = line[2].chars().nth(0).unwrap();
                    let val2 = registers.entry(reg2).or_insert(0);
                    *val2 = val1;
                }
            }
            i += 1;
        } else if cmd == "jnz" {
            let amt: usize;
            let neg: bool;
            if line[2].chars().nth(0).unwrap() == '-' {
                neg = true;
                amt = line[2][1..].parse().unwrap();
            } else {
                neg = false;
                amt = line[2].parse().unwrap();
            }
            let op = line[1].parse::<i32>();
            match op {
                Ok(num) => {
                    if num != 0 {
                        if neg {
                            i -= amt;
                        } else {
                            i += amt;
                        }
                    } else {
                        i += 1;
                    }
                }
                Err(..) => {
                    let reg = line[1].chars().nth(0).unwrap();
                    let val = registers.entry(reg).or_insert(0);
                    if *val != 0 {
                        if neg {
                            i -= amt;
                        } else {
                            i += amt;
                        }
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    println!("{}", registers.get(&'a').unwrap());
}