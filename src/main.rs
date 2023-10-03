use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const STR: &str = "Happy rustacean birthday, Speykious!";
const UPPER_ABC: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const LOWER_ABC: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn printf(astring: &String) {
    print!("{astring}\r");
    stdout().flush().unwrap();
    let five = Duration::from_millis(150);
    sleep(five);
}

fn pick_char(character: char) -> char {
    let mut rng = thread_rng();
    if character.is_uppercase() {
        UPPER_ABC[rng.gen_range(0..UPPER_ABC.len())]
    } else if character.is_whitespace() {
        ' '
    } else if character.is_ascii_punctuation() {
        character
    } else {
        LOWER_ABC[rng.gen_range(0..LOWER_ABC.len())]
    }
}

fn rand_text(length: usize) -> String {
    let mut accumulator = String::new();
    for i in 0..=length {
        let character = STR.chars().nth(i).unwrap();
        accumulator.push(pick_char(character));
    }
    accumulator
}

fn extract_char(string: &str, i: usize) -> char {
    string.chars().nth(i).unwrap()
}
fn main() {
    let mut string = String::new();

    for i in 0..STR.len() {
        string = rand_text(i);
        printf(&string);
    }
    while string != STR {
        for i in 0..string.len() {
            if extract_char(&string, i) == extract_char(STR, i) {
                continue;
            }
            let character = extract_char(STR, i);
            string.replace_range(i..=i, &pick_char(character).to_string());
        }
        printf(&string);
    }
    println!("{string}");
}
