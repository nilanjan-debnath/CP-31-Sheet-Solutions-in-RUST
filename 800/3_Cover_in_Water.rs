/*
problem link: https://codeforces.com/problemset/problem/1900/A
test input:
5
3
...
7
##....#
7
..#.#..
4
####
10
#...#..#.#

expected output:
2
2
5
0
2

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
        let _n: i32 = take_input().trim().parse().expect(" ");
        let s: String = take_input().trim().to_string();
        
        let mut dot: u32 = 0;
        let mut longest: u32 = 0;
        let mut tmp: u32 = 0;
        
        for x in s.chars() {
            if x == '.'{
                dot += 1;
                tmp += 1;
                if tmp > longest {
                    longest = tmp;
                }
            } else {
                tmp = 0;
            }
        }
        if longest > 2 {
            println!("2");
        } else {
            println!("{dot}");
        }
    }
}