fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut smallest = arr[0];
    for &number in arr.iter() {
        smallest = if number < smallest {number} else {smallest};
    }
    smallest
}