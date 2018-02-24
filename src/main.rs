extern crate sha1;
extern crate rpassword;

use std::io::{ Read, Seek, SeekFrom };
use std::fs::File;
use std::cmp::{ Ord, Ordering };
use std::env::args;

fn search<T: Read + Seek>(mut db: T, hash: &str) -> Option<usize> {
    let line_size = hash.len() + 1; // +1 for the linefeed
    let file_size = db.seek(SeekFrom::End(0)).expect("can't seek");
    let num_lines = file_size as usize / line_size;
    let mut buf = vec![0; hash.len()];
    let mut lower = 0;
    let mut upper = num_lines;

    // Binary search the sorted file
    while lower < upper {
        let middle = lower + (upper - lower) / 2;

        // Read the line
        db.seek(SeekFrom::Start(middle as u64 * line_size as u64)).expect("can't seek");
        db.read_exact(&mut buf).expect("can't read file");

        match hash.as_bytes().cmp(&buf) {
            Ordering::Equal   => return Some(middle),
            Ordering::Less    => upper = middle,
            Ordering::Greater => lower = middle + 1,
        }
    }

    None
}

fn main() {
    let db_fname = args().nth(1).unwrap_or(String::from("pwned-passwords-2.0-cut.txt"));
    let db = File::open(db_fname).expect("DB file");
    let pw = rpassword::prompt_password_stdout("\nPassword to search: ").expect("password");

    let hash = {
        let mut hash = sha1::Sha1::from(&pw).hexdigest();
        hash.make_ascii_uppercase();
        hash
    };

    println!("    Hash: {}\n", hash);

    match search(db, &hash) {
        Some(line) => println!("YOU ARE PWNED: HASH FOUND on line {}!!!", line),
        None => println!("Hash _not_ found - You are probably safe."),
    }

    println!();
}
