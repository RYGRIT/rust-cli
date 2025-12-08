use rand::{Rng, seq::SliceRandom};
use zxcvbn::zxcvbn;

use crate::opts::GenPassOpts;

const UPPER_CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CHARSET: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const DIGIT_CHARSET: &[u8] = b"123456789";
const SYMBOL_CHARSET: &[u8] = b"!@#$%^&*()_";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    println!("Generating password with length: {}", opts.length);
    let mut password = Vec::with_capacity(opts.length as usize);
    let mut chars = Vec::with_capacity(opts.length as usize);
    let mut rng = rand::rng();

    if opts.length < 6 {
        anyhow::bail!("Password length must be at least 6");
    }

    // Ensure at least one character from each selected set is included
    for (include, charset) in [
        (opts.uppercase, UPPER_CHARSET),
        (opts.lowercase, LOWER_CHARSET),
        (opts.numbers, DIGIT_CHARSET),
        (opts.special, SYMBOL_CHARSET),
    ] {
        if include {
            chars.extend_from_slice(charset);
            password.push(charset[rng.random_range(0..charset.len())]);
        }
    }

    if chars.is_empty() {
        anyhow::bail!("At least one character type must be selected");
    }

    while password.len() < opts.length as usize {
        let idx = rng.random_range(0..chars.len());
        password.push(chars[idx]);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8_lossy(&password);

    println!("Generated password: {}", password);

    let estimate = zxcvbn(&password, &[]);
    // output password strength in stderr
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
