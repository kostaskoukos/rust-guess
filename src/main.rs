use rand::Rng;

fn main() {
    println!("Welcome to the guessing game! I will think of a number in the range [1, 100] and you'll try to guess it");

    let secret = rand::thread_rng().gen_range(0..=100);
}
