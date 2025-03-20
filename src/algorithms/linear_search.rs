fn linear_search(arr: Vec<i32>, key: i32) -> bool {
    for i in 0..arr.len() {
        if arr[i] == key {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = vec![5, 4, 3, 2, 1];
        let key = 3;
        let result = linear_search(arr, key);
        assert!(result);
    }
}
