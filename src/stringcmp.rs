fn compare_string(x: &str, y: &str) -> bool {
    let x_bytes = x.as_bytes();
    let y_bytes = y.as_bytes();
    let min_len = std::cmp::min(x_bytes.len(), y_bytes.len());

    for i in 0..min_len {
        if x_bytes[i] < y_bytes[i] {
            return false;
        } else if x_bytes[i] > y_bytes[i] {
            return true;
        }
    }
    x_bytes.len() > y_bytes.len()
}

fn main() {
    let x = "apple";
    let y = "banana";

    let result = compare_string(x, y);
    if result {
        println!("'{}' is greater than '{}'", x, y);
    } else {
        println!("'{}' is not greater than '{}'", x, y);
    }
}
