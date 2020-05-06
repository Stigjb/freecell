use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextureAtlas {
    pub dimensions: Dimensions,
    pub locations: HashMap<String, Location>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Dimensions {
    pub w: usize,
    pub h: usize,
}
