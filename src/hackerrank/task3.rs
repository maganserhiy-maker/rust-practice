// https://www.hackerrank.com/challenges/grading

fn grading(grade: i32) -> i32 {
    if grade < 38 {
        return grade;
    }
    let next_multiple = ((grade / 5) + 1) * 5;
    if next_multiple - grade < 3 {
        next_multiple
    } else {
        grade
    }
}

#[test]
fn test3() {
    assert_eq!(grading(73), 75);
    assert_eq!(grading(67), 67);
    assert_eq!(grading(38), 40);
    assert_eq!(grading(33), 33);
}