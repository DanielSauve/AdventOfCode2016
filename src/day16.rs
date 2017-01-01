use std::str::FromStr;
use std::ops::Index;

fn main() {
    let len = 272;
    let mut a = String::from_str("00111101111101000").unwrap();
    while a.len() < len {
        let mut b = a.clone();
        unsafe {
            let mut vec = b.as_mut_vec();
            vec.reverse();
            for i in 0..vec.len(){
                if vec[i] == 49{
                    vec[i] = 48;
                } else {
                    vec[i] = 49;
                }
            }
        }
        println!("{:?}", a);
        println!("{:?}", b);
        a.push('0');
        a = a + &b;
        println!("{:?}", a);
    }

    let mut checksum = String::from_str(a.index(0..len)).unwrap();
    let mut new_checksum: String;
    while checksum.len() % 2 == 0 {
        new_checksum = String::new();
        let mut i = 0;
        println!("{}", checksum);
        while i < checksum.len() {
            if checksum[i..i + 1] == checksum[i + 1..i + 2] {
                print!("{}", String::from_str(&checksum[i..i + 1]).unwrap());
                println!("{}", String::from_str(&checksum[i + 1..i + 2]).unwrap());
                new_checksum.push('1');
            } else {
                print!("{}", String::from_str(&checksum[i..i + 1]).unwrap());
                println!("{}", String::from_str(&checksum[i + 1..i + 2]).unwrap());
                new_checksum.push('0');
            }
            i += 2;
        }
        checksum = new_checksum.clone();
    }
    println!("{}", checksum)
}