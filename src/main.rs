use rand::{thread_rng, Rng};
use std::io::{stdout, Write, self};
use std::thread::sleep;
use std::time::Duration;

const MESSAGE: &[u8] = b"Happy rustacean birthday, Speykious!";

fn print_message(astring: &[u8]) -> io::Result<()> {
    stdout().write_all(b"\r")?;
    stdout().write_all(astring)?;
    stdout().flush()?;

    sleep(Duration::from_millis(150));
    Ok(())
}

fn pick_random_char(character: u8) -> u8 {
    let mut rng = thread_rng();

    match character {
        c if c.is_ascii_uppercase() => rng.gen_range(b'A'..=b'Z'),
        c if c.is_ascii_whitespace() => b' ',
        c if c.is_ascii_punctuation() => c,
        _ => rng.gen_range(b'a'..=b'z'),
    }
}

fn rand_text(length: usize) -> Vec<u8> {
    (MESSAGE.iter())
        .take(length)
        .copied()
        .map(pick_random_char)
        .collect::<_>()
}

fn main() {
    let mut bogo_msg = Vec::new();

    for i in 0..MESSAGE.len() {
        bogo_msg = rand_text(i);
        print_message(&bogo_msg).unwrap();
    }

    while bogo_msg != MESSAGE {
        for (bogo_char, mess_char) in bogo_msg.iter_mut().zip(MESSAGE) {
            if bogo_char != mess_char {
                *bogo_char = pick_random_char(*mess_char);
            }
        }
        print_message(&bogo_msg).unwrap();
    }

    print_message(&bogo_msg).unwrap();
    println!();
}
