extern crate bjorn;
extern crate clap;

use std::fs;
use std::io;
use clap::*;

fn main() -> io::Result<()> {
    let matches = App::new("bjorn")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("FILEPATH")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("File path of the source code to interpret."))
        .get_matches();

    let input = fs::read_to_string(matches.value_of("FILEPATH").unwrap())?;

    bjorn::interpret(&input);
    Ok(())
}
