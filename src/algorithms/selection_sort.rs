fn selection_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..(n - 1) {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        let temp = arr[i];
        arr[i] = arr[min_idx];
        arr[min_idx] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn text_selection_sort() {
        let mut arr = vec![6, 5, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![2, 5, 6]);
    }
}
