use eyre::eyre;
use serde::Deserialize;

use crate::{edge::Edge, latlon::LatLon};

#[derive(Debug, Deserialize, Clone)]
pub struct MaybeStatue {
    title: String,
    r#where: String,
    link: String,
    lat: Option<f32>,
    lon: Option<f32>,
    address: Option<String>,
}

impl TryInto<Statue> for MaybeStatue {
    type Error = eyre::Error;

    fn try_into(self) -> Result<Statue, Self::Error> {
        if self.lat.is_none() || self.lon.is_none() || self.address.is_none() {
            return Err(eyre!(
                "Missing coordinates and address for statue: {}",
                self.title
            ));
        }

        Ok(Statue {
            title: self.title,
            r#where: self.r#where,
            link: self.link,
            lat: self.lat.unwrap(),
            lon: self.lon.unwrap(),
            address: self.address.unwrap(),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Statue {
    title: String,
    r#where: String,
    link: String,
    lat: f32,
    lon: f32,
    address: String,
}

impl Statue {
    pub fn latlon(&self) -> LatLon {
        self.into()
    }
}

impl Edge for Statue {
    fn weight(&self, node: &Self) -> f32 {
        self.latlon().weight(&node.latlon())
    }
}

impl Into<LatLon> for Statue {
    fn into(self) -> LatLon {
        LatLon::new(self.lat, self.lon)
    }
}

impl Into<LatLon> for &Statue {
    fn into(self) -> LatLon {
        LatLon::new(self.lat, self.lon)
    }
}
