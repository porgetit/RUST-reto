fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for (i, &item) in arr.iter().enumerate() {
        if i % 2 == 0 || i == 0 {
            result.push(item);
        }
    }
    result
}