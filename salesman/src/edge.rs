use crate::{
    latlon::{LatLon, StartingPoint},
    statue::Statue,
};

pub trait Edge {
    fn weight(&self, node: &Self) -> f32;
}

#[derive(Debug, Clone)]
pub enum NodeLatLon {
    StartingPoint(StartingPoint),
    Statue(Statue),
}

impl NodeLatLon {
    fn latlon(&self) -> LatLon {
        match self {
            NodeLatLon::StartingPoint(starting_point) => starting_point.latlon().clone(),
            NodeLatLon::Statue(statue) => statue.latlon(),
        }
    }
}

impl Edge for NodeLatLon {
    fn weight(&self, node: &Self) -> f32 {
        self.latlon().weight(&node.latlon())
    }
}
