use std::f32::consts::PI;

use crate::tsp::Edge;

const R: f32 = 6371e3; // in metres

pub const GRONLAND_TBANE: (f32, f32) = (59.912733, 10.761390);

pub struct LatLon {
    lat: f32,
    lon: f32,
}

impl LatLon {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }

    // Uses haversine formula to calculate distance in metres.
    pub fn calculate_distance_to(&self, target: &LatLon) -> i32 {
        let phi_1 = self.lat * (PI / 180.0);
        let phi_2 = target.lat * (PI / 180.0);

        let delta_phi = (target.lat - self.lat) * (PI / 180.0);
        let delta_lambda = (target.lon - self.lon) * (PI / 180.0);

        let a = ((delta_phi / 2.0).sin() * (delta_phi / 2.0).sin())
            + (phi_1.cos() * phi_2.cos() * (delta_lambda / 2.0).sin() * (delta_lambda / 2.0).sin());

        let c = 2.0 * (a.sqrt().atan2((1.0 - a).sqrt()));

        (R * c).floor() as i32 // in metres
    }
}

impl Edge for LatLon {
    fn weight(&self, node: &Self) -> i32 {
        self.calculate_distance_to(node)
    }
}

#[cfg(test)]
mod tests {
    use super::LatLon;

    #[test]
    fn test() {
        let point_1 = LatLon::new(59.8759717, 10.8297305);
        let point_2 = LatLon::new(59.9326106, 10.7344451);

        let distance = point_1.calculate_distance_to(&point_2);

        assert_eq!(distance, 8239)
    }
}
