// https://www.hackerrank.com/challenges/staircase/problem

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

#[test]
fn test2() {
    staircase(4);
}