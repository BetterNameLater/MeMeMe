use crate::math::vec2i::Vec2i;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::TypePath;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct MapRepr {
    pub map: Vec<Vec<BackgroundType>>,
    pub objects: HashMap<String, ObjectRepr>,
}

#[derive(Default)]
pub struct MapLoader;

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
            match serde_json::from_slice::<MapRepr>(&bytes) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum BackgroundType {
    Floor = 0,
    Wall = 1,
    Start = 2,
    End = 3,
}

#[derive(Deserialize, Debug)]
pub struct ObjectRepr {
    pub position: Vec2i,
    #[serde(default)]
    pub interaction_type: InteractionType,
    #[serde(rename = "data")]
    pub object_type: ObjectType,
    #[serde(default)]
    pub depends_on: HashMap<String, bool>,
    #[serde(default)]
    pub single_use: bool,
    #[serde(default)]
    pub killing: bool,
    #[serde(default)]
    pub start_timer: Option<f32>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    #[default]
    All,
    GhostOnly,
    PlayerOnly,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ObjectType {
    PressurePlate,
    PressurePlateOnOff,
    Door,
    Teleporter { destination: Vec2i },
    LevelTeleporter { destination: String },
    Lever,
    Button, // TODO avec le temps de cool-down variable ?
}
