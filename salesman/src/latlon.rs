use std::{f32::consts::PI, sync::LazyLock};

use crate::edge::Edge;

const R: f32 = 6371e3; // in metres

pub const GRONLAND_TBANE: LazyLock<StartingPoint> = LazyLock::new(|| {
    StartingPoint::new(
        LatLon::new(59.912_73, 10.761_39),
        "Grønland T-bane".to_string(),
    )
});

pub const KAMPEN: LazyLock<StartingPoint> =
    LazyLock::new(|| StartingPoint::new(LatLon::new(59.913_34, 10.774_524), "Kampen".to_string()));

#[derive(Debug, Clone)]
pub struct StartingPoint {
    latlon: LatLon,
    r#where: String,
}

impl StartingPoint {
    fn new(latlon: LatLon, r#where: String) -> Self {
        Self { latlon, r#where }
    }

    pub fn latlon(&self) -> &LatLon {
        &self.latlon
    }
}

impl Edge for StartingPoint {
    fn weight(&self, node: &Self) -> f32 {
        self.latlon.weight(&node.latlon)
    }
}

#[derive(Debug, Clone)]
pub struct LatLon {
    lat: f32,
    lon: f32,
}

impl LatLon {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }

    // Uses haversine formula to calculate distance in metres.
    pub fn calculate_distance_to(&self, target: &LatLon) -> f32 {
        let phi_1 = self.lat * (PI / 180.0);
        let phi_2 = target.lat * (PI / 180.0);

        let delta_phi = (target.lat - self.lat) * (PI / 180.0);
        let delta_lambda = (target.lon - self.lon) * (PI / 180.0);

        let a = ((delta_phi / 2.0).sin() * (delta_phi / 2.0).sin())
            + (phi_1.cos() * phi_2.cos() * (delta_lambda / 2.0).sin() * (delta_lambda / 2.0).sin());

        let c = 2.0 * (a.sqrt().atan2((1.0 - a).sqrt()));

        (R * c).floor() // in metres
    }
}

impl Edge for LatLon {
    fn weight(&self, node: &Self) -> f32 {
        self.calculate_distance_to(node)
    }
}

#[cfg(test)]
mod tests {
    use super::LatLon;

    #[test]
    fn test() {
        let point_1 = LatLon::new(59.875_973, 10.829_73);
        let point_2 = LatLon::new(59.932_61, 10.734_446);

        let distance = point_1.calculate_distance_to(&point_2);

        assert_eq!(distance, 8239.0)
    }
}
