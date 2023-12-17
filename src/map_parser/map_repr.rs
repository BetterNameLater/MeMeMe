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
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(serde_json::from_slice::<MapRepr>(&bytes).unwrap())
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
    pub position: Vec2i,
    pub destination: Option<Vec2i>,

    #[serde(default)]
    pub start_state: bool,

    #[serde(rename = "type")]
    pub object_type: ObjectType,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub ghost_only: bool,
    #[serde(default)]
    pub player_only: bool,
    #[serde(default)]
    pub single_use: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    PressurePlate,
    Door,
    Teleporter,
	Lever,
}
