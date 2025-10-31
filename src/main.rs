use std::io::{Read, stdin};

use anyhow::Result;
use ccs::StatusLine;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    println!("{}", StatusLine::try_from(input.as_str())?);
    Ok(())
}
