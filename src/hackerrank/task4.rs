// https://www.hackerrank.com/challenges/apple-and-orange

fn apple_and_orange(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter().filter(|&&x| {
        let pos = a + x;
        pos >= s && pos <= t
    }).count() as i32;

    let orange_count = oranges.iter().filter(|&&x| {
        let pos = b + x;
        pos >= s && pos <= t
    }).count() as i32;

    (apple_count, orange_count)
}

#[test]
fn test4() {
    let (apples, oranges) = apple_and_orange(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
    assert_eq!(apples, 1);
    assert_eq!(oranges, 1);
}