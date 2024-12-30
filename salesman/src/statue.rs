use eyre::eyre;
use serde::Deserialize;

use crate::latlon::LatLon;

#[derive(Debug, Deserialize, Clone)]
pub struct Statue {
    title: String,
    r#where: String,
    link: String,
    lat: Option<f32>,
    lon: Option<f32>,
    address: Option<String>,
}

impl Statue {
    pub fn has_pos(&self) -> bool {
        if self.lat.is_none() || self.lon.is_none() {
            return false;
        }

        return true;
    }
}

impl TryInto<LatLon> for Statue {
    type Error = eyre::Error;

    fn try_into(self) -> Result<LatLon, Self::Error> {
        if !self.has_pos() {
            return Err(eyre!("Missing gps coordinates for statue: {}", self.title));
        }

        Ok(LatLon::new(self.lat.unwrap(), self.lon.unwrap()))
    }
}

impl TryInto<LatLon> for &Statue {
    type Error = eyre::Error;

    fn try_into(self) -> Result<LatLon, Self::Error> {
        if !self.has_pos() {
            return Err(eyre!("Missing gps coordinates for statue: {}", self.title));
        }

        Ok(LatLon::new(self.lat.unwrap(), self.lon.unwrap()))
    }
}
