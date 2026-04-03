use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,  Debug)]
pub struct AssetIndex {
    /// Map of asset objects: the key is the path (e.g., "minecraft/sounds/ambient/cave/cave1.ogg")
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Deserialize, Serialize,  Debug)]
pub struct AssetObject {
    /// The hash of the asset file, used to locate it in the assets directory (e.g., "3a7bd3c9e5f1a2b4c6d8e9f0a1b2c3d4e5f6g7")
    pub hash: String,
    pub size: u64,
}