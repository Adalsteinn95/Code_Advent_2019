pub fn one(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers.iter() {
        sum += number / 3 - 2;
    }

    return sum;
}
