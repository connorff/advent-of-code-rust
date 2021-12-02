mod day01;

fn main() {
    let submarine_increases = day01::submarine();
    println!("submarine found {} increases", submarine_increases);

    let submarine_window_increases = day01::window_submarine();
    println!(
        "submarine found {} increases with sliding windows",
        submarine_window_increases
    );
}
