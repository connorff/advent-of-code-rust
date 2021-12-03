mod day03;

fn main() {
    let gamma = day03::gamma();
    let epsilon = day03::epsilon(gamma.clone());

    let gamma_int = day03::to_int(gamma.clone());
    let epsilon_int = day03::to_int(epsilon);

    println!("{:?}", gamma_int * &epsilon_int)
}
