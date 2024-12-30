use std::{cmp::Ordering, env};

use latlon::{LatLon, GRONLAND_TBANE};
use statue::Statue;

mod latlon;
mod statue;
mod tsp;

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Send in path to file plis");
    }

    let path = args[1].clone();

    let mut statues: Vec<Statue> = get_from_path(path)?
        .into_iter()
        .filter(|val| val.has_pos())
        .collect();

    println!("before sort:\n{:#?}", &statues[..3]);
    let start = LatLon::new(GRONLAND_TBANE.0, GRONLAND_TBANE.1);

    // Sort statues by proximity to start point for testing
    statues.sort_by(|a: &Statue, b: &Statue| -> Ordering {
        let a_pos: LatLon = a.try_into().unwrap();
        let b_pos: LatLon = b.try_into().unwrap();

        let diff = a_pos.calculate_distance_to(&start) - b_pos.calculate_distance_to(&start);

        if diff == 0 {
            return Ordering::Equal;
        } else if diff < 0 {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    println!("after sort:\n{:#?}", &statues[..3]);

    Ok(())
}

fn get_from_path(path: String) -> eyre::Result<Vec<Statue>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut output = vec![];

    for record in rdr.deserialize() {
        let record: Statue = record?;
        output.push(record);
    }

    Ok(output)
}
