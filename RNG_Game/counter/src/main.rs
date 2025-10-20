fn main() {
    let mut current_count: i8 = 0;
    let stop_count: i8 = 20;

    while current_count <= stop_count {
        println!("Counting: {current_count}");
        current_count += 1;
    }
}
