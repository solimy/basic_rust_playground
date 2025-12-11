fn average(numbers: &[i32]) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_average() {
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(average(&nums), 3.0);
    }
}
