fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let n1 = mid - left;
    let n2 = right - mid;
    let  l1 = arr.clone();
    let  r1 = arr.clone();
    let l = &l1[left..mid];
    let r = &r1[mid..right];

    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < n1 && j < n2 {
        if l[i] < r[j] {
            arr[k] = l[i];
            i = i + 1;
        } else {
            arr[k] = r[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = l[i];
        i = i + 1;
        k = k + 1;
    }

    while j < n2 {
        arr[k] = r[j];
        j = j + 1;
        k = k + 1;
    }
    arr
}

fn merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

fn main() {
    let mut arr: Vec<i32> = vec![94, 1, 119, -87, 50, 43, 48];
    arr = merge_sort(arr.clone(), 0, arr.len());
    println!("Sorted array is {:?}", arr);
}