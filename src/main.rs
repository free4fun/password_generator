use rand::Rng;

fn main() {
    println!("Generating password...");
    let rnd_string = random_string(32);
    println!("{}", rnd_string);
}

fn random_char() -> char {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=92);
    let char_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_-=+[].,<>;:%¿?¡ºª/|\\";
    char_set.chars().nth(random_number).unwrap()
}

fn random_string(num: i32) -> String {
    return (0..num).map(|_| random_char()).collect();
}