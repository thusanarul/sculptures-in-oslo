use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Statue {
    title: String,
    r#where: String,
    link: String,
    lat: Option<f32>,
    lon: Option<f32>,
    address: Option<String>,
}

pub type Statues = Vec<Statue>;
