use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, serde::Deserialize)]
pub enum OsmObject {
    #[serde(rename = "osm-node-id")]
    Node(usize),

    #[serde(rename = "osm-way-id")]
    Way(usize),
}

impl Display for OsmObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node(id) => write!(f, "https://www.openstreetmap.org/node/{}", id),
            Self::Way(id) => write!(f, "https://www.openstreetmap.org/way/{}", id),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Bar {
    pub name: String,

    #[serde(flatten)]
    pub osm: OsmObject,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BarsFile {
    pub bars: Vec<Bar>,
}
