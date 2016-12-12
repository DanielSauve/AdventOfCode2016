use std::fs::File;
use std::io::prelude::Read;
use std::ops::Index;

fn main(){
    let mut array = [[false; 50]; 6];
    let mut input = String::new();
    let mut file = File::open("../resources/day8.txt").unwrap();
    match file.read_to_string(&mut input){
        Err(e) => panic!("Could not open file: {}", e),
        Ok(_) => ()
    }
    for i in input.lines(){
        if i.contains("rect"){
            rect(i, &mut array);
        }
        else if i.contains("row"){
            let rows = i.index(13..14);
            let amount = i.index(18..);
            row(rows.parse().unwrap(), amount.parse().unwrap(), &mut array);
        }
        else if i.contains("column"){
            let mut iter = i.split_whitespace();
            iter.next();
            iter.next();
            let text = iter.next().unwrap();
            let columns : &str;
            if text.len() == 3{
                columns = text.index(2..3);
            } else {
                columns = text.index(2..4);
            }
            iter.next();
            let amount = iter.next().unwrap();
            column(columns.parse().unwrap(), amount.parse().unwrap(), &mut array);
        }
    }
    let mut count = 0;
    for i in 0..6{
        for j in 0..50{
            if array[i][j] == true{
                print!("#");
                count += 1;
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("{}", count)
}

fn rect(input: &str, array : &mut [[bool;50];6]){
    let test = input.as_bytes();
    let rows : usize;
    let columns : usize;
    if test[7] == b'x'{
        columns = ((test[5] -48) * 10 + (test[6] -48)) as usize;
        rows = (test[8] - 48) as usize;
    }else{
        columns = (test[5] - 48) as usize;
        rows = (test[7] - 48) as usize;
    }
    for i in 0..rows{
        for j in 0..columns{
            array[i][j] = true;
        }
    }
}

fn row(row: usize, amount: usize, array : &mut [[bool;50];6]){
    let mut new_row = [false; 50];
    for i in 0..50{
        new_row[(i+amount)%50] = array[row][i].clone();
    }
    for i in 0..50{
        array[row][i] = new_row[i].clone();
        assert_eq!(array[row][i], new_row[i]);
    }
}

fn column(column: usize, amount: usize, array : &mut [[bool;50];6]){
    let mut new_column = [false ; 6];
    for i in 0..6{
        new_column[(i+amount)%6] = array[i][column].clone();
    }
    for i in 0..6{
        array[i][column] = new_column[i].clone();
        assert_eq!(array[i][column], new_column[i]);
    }
}
