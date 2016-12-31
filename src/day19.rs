//Day 19 part 1 was just the Josephus problem, so this is easy to implement with bitwise operations

fn main() {
    let input : u32 = 3014603;
    let leading_zeros = input.leading_zeros();
    let first_one = 2u32.pow(31-leading_zeros);
    let output = ((input - first_one) << 1) + 1;
    println!("{}", output);
}