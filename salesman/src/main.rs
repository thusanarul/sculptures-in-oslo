use std::env;

use statue::{Statue, Statues};

mod statue;

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Send in path to file plis");
    }

    let path = args[1].clone();

    let statues = get_from_path(path)?;

    println!("{:?}", &statues[..3]);

    Ok(())
}

fn get_from_path(path: String) -> eyre::Result<Statues> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut output: Statues = vec![];

    for record in rdr.deserialize() {
        let record: Statue = record?;
        output.push(record);
    }

    Ok(output)
}
