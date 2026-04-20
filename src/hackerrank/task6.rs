pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    
    let start = *a.iter().max().unwrap_or(&1);
    let end = *b.iter().min().unwrap_or(&100);

    for x in start..=end {
        let fits_a = a.iter().all(|&ai| x % ai == 0);
        let fits_b = b.iter().all(|&bi| bi % x == 0);

        if fits_a && fits_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(getTotalX(&a, &b), 3);
    }

    #[test]
    fn test_simple_case() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(getTotalX(&a, &b), 2);
    }
}