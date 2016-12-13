use std::fs::File;
use std::io::prelude::Read;
use std::ops::Index;

fn main() {
    let mut input = String::new();
    let mut file = File::open("../resources/day9.txt").unwrap();
    match file.read_to_string(&mut input){
        Err(e) => panic!("Could not open file: {}", e),
        Ok(_) => ()
    }
    let mut decompressed = String::new();
    while input.len()>0{
        let index = input.find("(");
        match index {
            Some(_) => (),
            None => break
        }
        let index = index.unwrap();
        let index2 : usize;
        let end_index = input.find(")").unwrap();
        let num_chars : usize;
        {
            let (first, second) = input.split_at(index);
            for c in first.chars(){
                decompressed.push(c);
            }
            index2 = second.find(")").unwrap();
            let (decode, _) = second.split_at(index2);
            let x_index = decode.find("x").unwrap();
            num_chars = decode.index(1..x_index).parse().unwrap();
            let num_repeat : usize = decode.index(x_index+1..).parse().unwrap();
            let repeat_chars = input.index(end_index+1..end_index+num_chars+1);
            for _ in 0..num_repeat{
                decompressed.push_str(repeat_chars);
            }
        }
        input = input.index(end_index+1+num_chars..).to_string();
    }
    println!("{}", decompressed);
    println!("{}", decompressed.len());
}
