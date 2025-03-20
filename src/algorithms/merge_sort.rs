fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
    let size_of_left = mid - left + 1;
    let size_of_right = right - mid;

    let mut temp_left = vec![0; size_of_left];
    let mut temp_right = vec![0; size_of_right];

    for i in 0..size_of_left {
        temp_left[i] = arr[left + i];
    }
    for j in 0..size_of_right {
        temp_right[j] = arr[mid + 1 + j];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < size_of_left && j < size_of_right {
        if temp_left[i] < temp_right[j] {
            arr[k] = temp_left[i];
            i += 1;
        } else {
            arr[k] = temp_right[j];
            j += 1;
        }
        k += 1;
    }

    while i < size_of_left {
        arr[k] = temp_left[i];
        i += 1;
        k += 1;
    }

    while j < size_of_right {
        arr[k] = temp_right[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        merge_sort(arr, left, mid);
        merge_sort(arr, mid + 1, right);
        merge(arr, left, mid, right);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let size = arr.len();
        merge_sort(&mut arr, 0, size - 1);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
