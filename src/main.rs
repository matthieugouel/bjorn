extern crate bjorn;

use std::env;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "Please enter the file path.")
        );
    }

    let input = fs::read_to_string(&args[1])?;

    println!("{}", bjorn::interpret(&input));
    Ok(())

}
