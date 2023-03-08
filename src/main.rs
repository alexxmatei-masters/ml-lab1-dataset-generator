use rand::rngs::ThreadRng;
use rand::Rng;

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    let random_number: i16 = rng.gen_range(1..=10);
    println!("Random number between 1 and 10: {}", random_number);
}
