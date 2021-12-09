use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_int: u32 = rng.gen_range(0, 10);
    println!("{}", random_int);
}
