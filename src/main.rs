mod day02;

fn main() {
    let (mult_movement, mult_movement_aim) = day02::travel();
    println!("the submarine traveled x * y: {}", mult_movement);
    println!(
        "the submarine traveled x * y with aim: {}",
        mult_movement_aim
    );
}
