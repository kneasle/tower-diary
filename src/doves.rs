use std::{
    fmt::{Display, Formatter},
    path::Path,
};

use serde::{Deserialize, Deserializer};

pub fn read_doves_csv(path: impl AsRef<Path>) -> Vec<Ring> {
    let mut rings = Vec::new();
    let mut reader = csv::Reader::from_path(path).unwrap();
    for result in reader.deserialize() {
        let ring: Ring = result.unwrap();
        rings.push(ring);
    }
    rings
}

/// A `Ring` of bells in Dove's Guide.  Note that the same tower could contain multiple `Ring`s.
#[derive(Debug, Clone, Deserialize)]
pub struct Ring {
    #[serde(rename = "TowerID")]
    pub id: usize,
    #[serde(rename = "RingType")]
    pub ring_type: RingType,

    #[serde(rename = "Dedicn")]
    pub dedication: String, // TODO: Is this optional?
    #[serde(rename = "Place")]
    pub place: String,
    #[serde(rename = "Place2")]
    pub place2: Option<String>,
    #[serde(rename = "County")]
    pub county: Option<String>,
    #[serde(rename = "Country")]
    pub country: String,

    #[serde(rename = "AltName")]
    pub alt_name: Option<String>,

    #[serde(rename = "Wt", deserialize_with = "deser_weight")]
    pub weight: Weight,
    #[serde(rename = "Bells")]
    pub num_bells: usize,

    #[serde(rename = "UR", deserialize_with = "deser_empty")]
    pub ringable: bool,
}

/// The possible types of a rings documented in Dove's Guide.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum RingType {
    #[serde(rename = "Full circle ring")]
    FullCircle,
    Carillon,
}

/// The `Weight` of the heaviest bell in a [`Ring`]
///
/// TODO: Add more methods for this
#[derive(Debug, Clone)]
pub struct Weight {
    lbs: f64,
}

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cwt = self.lbs / 14.0 / 8.0;
        let kg = self.lbs * 0.453592;

        write!(f, "{cwt:.2} cwt / {kg:.0}kg")
    }
}

/////////////////////////////
// DESERIALIZATION HELPERS //
/////////////////////////////

/// Serializes as `false` if the next string is empty and `true` otherwise.
fn deser_not_empty<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    <&str>::deserialize(de).map(|s| !s.is_empty())
}

/// Serializes as `false` if the next string is empty and `true` otherwise.
fn deser_empty<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    <&str>::deserialize(de).map(str::is_empty)
}

/// Serializes the next string as a weight in pounds
fn deser_weight<'de, D>(de: D) -> Result<Weight, D::Error>
where
    D: Deserializer<'de>,
{
    f64::deserialize(de).map(|lbs| Weight { lbs })
}
