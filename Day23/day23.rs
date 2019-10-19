use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("day23.txt").unwrap();
    match file.read_to_string(&mut input) {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(_) => (),
    }
    let mut toggles: HashSet<i64> = HashSet::new();
    let mut registers: HashMap<char, i64> = HashMap::with_capacity(4);
    registers.insert('a', 7);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    let assembly: Vec<&str> = input.lines().collect();
    let mut i = 0;
    while i < assembly.len() {
        let line: Vec<&str> = assembly[i].split(" ").collect();
        let cmd = line[0];
        if cmd == "inc" {
            if toggles.contains(&(i as i64)) {
                toggles.remove(&(i as i64));
                let reg = line[1].chars().nth(0).unwrap();
                let val = registers.entry(reg).or_insert(0);
                *val -= 1;
                i += 1;
            } else {
                let reg = line[1].chars().nth(0).unwrap();
                let val = registers.entry(reg).or_insert(0);
                *val += 1;
                i += 1;
            }
        } else if cmd == "dec" {
            if toggles.contains(&(i as i64)) {
                toggles.remove(&(i as i64));
                let reg = line[1].chars().nth(0).unwrap();
                let val = registers.entry(reg).or_insert(0);
                *val += 1;
                i += 1;
            } else {
                let reg = line[1].chars().nth(0).unwrap();
                let val = registers.entry(reg).or_insert(0);
                *val -= 1;
                i += 1;
            }
        } else if (cmd == "cpy" && !toggles.contains(&(i as i64))) ||
                  (cmd == "jnz" && toggles.contains(&(i as i64))) {
            toggles.remove(&(i as i64));
            let op = line[1].parse::<i64>();
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
        } else if (cmd == "jnz" && !toggles.contains(&(i as i64))) ||
                  (cmd == "cpy" && toggles.contains(&(i as i64))) {
            toggles.remove(&(i as i64));
            let amt: usize;
            let neg: bool;
            if line[2].chars().nth(0).unwrap() == '-' {
                neg = true;
                amt = line[2][1..].parse().unwrap();
            } else {
                amt = match line[2].parse::<usize>() {
                    Ok(num) => {
                        neg = false;
                        num
                    }
                    Err(..) => {
                        let mut val = *registers.get(&line[2].chars().nth(0).unwrap()).unwrap();
                        if val < 0 {
                            neg = true;
                            val = -val;
                        } else {
                            neg = false;
                        }
                        val as usize
                    }
                }
            }
            let op = line[1].parse::<i64>();
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
                    println!("{}", *val);
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
        } else if cmd == "tgl" {
            if toggles.contains(&(i as i64)) {
                toggles.remove(&(i as i64));
                let reg = line[1].chars().nth(0).unwrap();
                let val = registers.entry(reg).or_insert(0);
                *val += 1;
                i += 1;
            } else {
                let reg = line[1].chars().nth(0).unwrap();
                let val = registers.entry(reg).or_insert(0);
                let push = *val + (i as i64);
                toggles.insert(push);
                println!("{:?}", push);
                i += 1;
            }
        }
    }
    println!("{}", registers.get(&'a').unwrap());
}