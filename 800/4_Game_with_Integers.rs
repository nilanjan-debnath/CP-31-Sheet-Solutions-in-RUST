/*
problem link: https://codeforces.com/problemset/problem/1899/A
test input:
6
1
3
5
100
999
1000

expected output:
First
Second
First
First
Second
First

*/
use std::io;

fn take_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(" ");
    return input;
}

fn main() {
    
    let t: i32 = take_input().trim().parse().expect(" ");
    
    for _ in 0..t {
        let n: i32 = take_input().trim().parse().expect(" ");
        if n%3 == 0 {
            println!("Second");
        } else {
            println!("First");
        }
    }
}
