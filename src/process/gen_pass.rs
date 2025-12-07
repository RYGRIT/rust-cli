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

    if opts.uppercase {
        chars.extend_from_slice(UPPER_CHARSET);
        password.push(UPPER_CHARSET[rng.random_range(0..UPPER_CHARSET.len())]);
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWER_CHARSET);
        password.push(LOWER_CHARSET[rng.random_range(0..LOWER_CHARSET.len())]);
    }

    if opts.numbers {
        chars.extend_from_slice(DIGIT_CHARSET);
        password.push(DIGIT_CHARSET[rng.random_range(0..DIGIT_CHARSET.len())]);
    }

    if opts.special {
        chars.extend_from_slice(SYMBOL_CHARSET);
        password.push(SYMBOL_CHARSET[rng.random_range(0..SYMBOL_CHARSET.len())]);
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

    // output password strength in stderr
    let estimate = zxcvbn(&password, &[]);
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
