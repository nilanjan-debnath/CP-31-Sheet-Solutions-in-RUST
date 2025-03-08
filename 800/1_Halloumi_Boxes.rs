/* 
problem link: https://codeforces.com/problemset/problem/1903/A
test input:
5
3 2
1 2 3
3 1
9 9 9
4 4
6 4 2 1
4 3
10 3 830 14
2 1
3 1

expected output:
YES
YES
YES
YES
NO
*/ 

#![allow(unused)] // as we are not going to use the array, added this attribute to avoid the warning
use std::io;
 
fn sort_check(arr: &Vec<i32>) -> bool {
    let n: usize = arr.len();
    for i in 1..n {
        if arr[i-1] > arr[i] {
            return false;
        }
    }
    return true;
}
 
fn main() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("");
    let mut t: i32 = input.trim().parse().expect("");
    while t>0 {
        t -= 1;
        
        let mut n: usize = 0;
        let mut k: usize = 0;
        let mut arr: Vec<i32> = Vec::new();
        {
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("");
            let mut tmp: Vec<usize> = Vec::new();
            for i in input.trim().split(" ") {
                let j: usize = i.parse().expect("");
                tmp.push(j);
            }
            n = tmp[0];
            k = tmp[1];
        }
        {
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("");
            for i in input.trim().split(" ") {
                let j: i32 = i.parse().expect("");
                arr.push(j);
            }
        }
        if k > 1 {
            println!("YES");
        } else {
            if sort_check(&arr) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}