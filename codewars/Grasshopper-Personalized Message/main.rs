fn greet(name: &str, owner: &str) -> String {
    return if name == owner {"Hello boss".to_string()} else {"Hello guest".to_string()};
}