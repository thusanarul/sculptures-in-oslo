use std::{cmp::Ordering, env, ops::Deref};

use edge::NodeLatLon;
use latlon::{LatLon, GRONLAND_TBANE, KAMPEN};
use mst::MST;
use statue::{MaybeStatue, Statue};
use tsp::TSP;

mod edge;
mod latlon;
mod mst;
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

    let start = GRONLAND_TBANE.deref().clone();

    // Sort statues by proximity to start point for testing to something close to home<3
    statues.sort_by(|a: &Statue, b: &Statue| -> Ordering {
        let a_pos: LatLon = a.latlon();
        let b_pos: LatLon = b.latlon();

        let diff = a_pos.calculate_distance_to(start.latlon())
            - b_pos.calculate_distance_to(start.latlon());

        if diff == 0.0 {
            return Ordering::Equal;
        } else if diff < 0.0 {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    let mut path: Vec<NodeLatLon> = vec![NodeLatLon::StartingPoint(start)];

    path.append(
        &mut statues[0..25]
            .iter()
            .map(|s| NodeLatLon::Statue(s.clone()))
            .collect::<Vec<NodeLatLon>>(),
    );

    let mut mst = MST::new(path.clone());
    mst.solve();
    let mst_lower_bound = mst.calculate_cost();

    let mut tsp = TSP::new(path);
    // let mut tsp = TSP::new_and_initialize_path(statues[0..20].to_vec());
    tsp.nn();
    tsp.three_opt();

    println!("Path:\n{:#?}", tsp.path());
    let tsp_cost = tsp.calculate_path_cost();
    println!("Total distance: {}", tsp_cost);

    println!("MST lower bound: {}", mst_lower_bound);
    println!(
        "Calculated distance to lower bound ratio: {}",
        tsp_cost / mst_lower_bound
    );

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
