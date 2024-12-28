use std::f32::consts::PI;

const R: f32 = 6371e3; // in metres

struct LatLon {
    lat: f32,
    lon: f32,
}

impl LatLon {
    fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }

    // Uses haversine formula to calculate distance in metres.
    fn calculate_distance_to(&self, target: &LatLon) -> f32 {
        let phi_1 = self.lat * (PI / 180.0);
        let phi_2 = target.lat * (PI / 180.0);

        let delta_phi = (target.lat - self.lat) * (PI / 180.0);
        let delta_lambda = (target.lon - self.lon) * (PI / 180.0);

        let a = ((delta_phi / 2.0).sin() * (delta_phi / 2.0).sin())
            + (phi_1.cos() * phi_2.cos() * (delta_lambda / 2.0).sin() * (delta_lambda / 2.0).sin());

        let c = 2.0 * (a.sqrt().atan2((1.0 - a).sqrt()));

        R * c // in metres
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

        assert_eq!(distance, 8239.428)
    }
}
