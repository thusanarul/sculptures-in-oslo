use std::{cmp::Ordering, env};

use latlon::{LatLon, GRONLAND_TBANE};
use statue::{MaybeStatue, Statue};
use tsp::TSP;

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
        .filter_map(|val| val.try_into().ok())
        .collect();

    let start = LatLon::new(GRONLAND_TBANE.0, GRONLAND_TBANE.1);

    // Sort statues by proximity to start point for testing to something close to home<3
    statues.sort_by(|a: &Statue, b: &Statue| -> Ordering {
        let a_pos: LatLon = a.latlon();
        let b_pos: LatLon = b.latlon();

        let diff = a_pos.calculate_distance_to(&start) - b_pos.calculate_distance_to(&start);

        if diff == 0.0 {
            return Ordering::Equal;
        } else if diff < 0.0 {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    let mut tsp = TSP::new(statues[0..10].to_vec());
    tsp.nn();
    tsp.three_opt();

    println!("Path:\n{:#?}", tsp.path());
    println!("Total distance: {}", tsp.calculate_path_cost());

    Ok(())
}

fn get_from_path(path: String) -> eyre::Result<Vec<MaybeStatue>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut output = vec![];

    for record in rdr.deserialize() {
        let record: MaybeStatue = record?;
        output.push(record);
    }

    Ok(output)
}
