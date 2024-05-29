use rand::Rng;

pub fn main(len : u128, choice:&str) -> String{
    let ascii_charset: Vec<char> = (33..127).map(|c| c as u8 as char).collect();
    let non_ascii_chars = vec!['á', 'é', 'í', 'ó', 'ú', 'ü', 'ñ', 'Ñ', 'ç', 'Ç', 'ß', 'Ø', 'ø', 'å', 'Å', 'æ', 'Æ', 'œ', 'Œ'];
    let all_charset: Vec<char> = ascii_charset.iter().chain(non_ascii_chars.iter()).cloned().collect();
    // Select the charset
    let char_set = match choice {
        "1" => &ascii_charset,
        "2" => &all_charset,
        _ => {
            println!("Invalid option. Using ASCII only charset by default.");
            &ascii_charset
        }
    };

    // Generates random string of charset
    let random_str = random_string(len, char_set);
    return random_str 
}

fn random_char(char_set: &[char]) -> char {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..char_set.len());
    char_set[random_index]
}

pub fn random_string(num: u128, char_set: &[char]) -> String {
    (0..num).map(|_| random_char(char_set)).collect()
}