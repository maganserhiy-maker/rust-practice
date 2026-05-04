// https://www.hackerrank.com/challenges/migratory-birds/problem
pub fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    let mut pairs = 0;

    for &sock in ar.iter().take(n as usize) {
        let count = counts.entry(sock).or_insert(0);
        *count += 1;
    }

    for &count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_example1() {
        let n = 9;
        let ar = [10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(n, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_example2() {
        let n = 7;
        let ar = [1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sock_merchant(n, &ar), 2);
    }

    #[test]
    fn test_no_pairs() {
        let ar = [1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }

    #[test]
    fn test_all_pairs() {
        let ar = [1, 1, 2, 2, 3, 3];
        assert_eq!(sock_merchant(6, &ar), 3);
    }
}