mod generator;

pub fn print_random_number() {
    let n = generator::gen_rand();

    println!("Random number u8: {}", n);
}
