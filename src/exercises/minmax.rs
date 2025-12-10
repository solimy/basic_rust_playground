fn min(numbers: &[i32]) -> i32 {
    todo!()
}

fn max(numbers: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]mod tests {
    use super::*;
    #[test]
    fn test_min() {
        let nums = [3, 1, 4, 1, 5];
        assert_eq!(min(&nums), 1);
    }

    #[test]
    fn test_max() {
        let nums = [3, 1, 4, 1, 5];
        assert_eq!(max(&nums), 5);
    }
}