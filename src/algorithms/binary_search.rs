fn binary_search(arr: Vec<i32>, key: i32) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == key {
            return Some(arr[mid]);
        }

        if arr[mid] > key {
            high = mid.saturating_sub(1);
        } else {
            low = mid + 1;
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        let key = 7;
        let result = binary_search(arr, key);
        assert_eq!(result, Some(7));
    }
}
