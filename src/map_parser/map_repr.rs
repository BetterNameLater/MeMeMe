use crate::math::vec2i::Vec2i;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::TypePath;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::io::ErrorKind;

#[derive(Asset, TypePath, Deserialize)]
pub struct MapRepr {
    pub map: Vec<Vec<BackgroundType>>,
    pub objects: HashMap<String, ObjectRepr>,
}

#[derive(Default)]
pub struct MapLoader;

#[non_exhaustive]
#[derive(Debug)]
pub enum CustomAssetLoaderError {
    Io(std::io::Error),
}
impl AssetLoader for MapLoader {
    type Asset = MapRepr;
    type Settings = ();
    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Err(std::io::Error::last_os_error())
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            // let custom_asset = ron::de::from_bytes::<CustomAsset>(&bytes)?;
            Ok(MapRepr {
                map: vec![vec![]],
                objects: HashMap::new(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum BackgroundType {
    Floor = 0,
    Wall = 1,
    Start = 2,
    End = 3,
}

#[derive(Deserialize)]
pub struct ObjectRepr {
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub position: Vec2i,
    #[serde(skip)]
    pub depends_on: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    PressurePlate,
    Door,
}
