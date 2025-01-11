use crate::math::vec2i::Vec2i;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::prelude::TypePath;
use schemars::{schema_for, JsonSchema, Schema};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Asset, TypePath, Deserialize, Debug, JsonSchema)]
pub struct MapRepr {
    pub map: Vec<Vec<BackgroundType>>,
    pub objects: HashMap<String, ObjectRepr>,
}

impl MapRepr {
    pub fn json_schema() {
        let schema: Schema = schema_for!(MapRepr);
        let mut v = schema.as_value().clone();
        *v.get_mut("$defs")
            .unwrap()
            .get_mut("BackgroundType")
            .unwrap() = json!({
            "enum": [0,1,2,3,4]
        });

        let file = File::create("level_schema.json").expect("haha");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &v).expect("haha");
        writer.flush().expect("haha");
    }
}

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
    type Asset = MapRepr;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        match serde_json::from_slice::<MapRepr>(&bytes) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Deserialize_repr, Debug, PartialEq, JsonSchema)]
#[repr(u8)]
pub enum BackgroundType {
    Floor = 0,
    Wall = 1,
    Start = 2,
    End = 3,
}

#[derive(Deserialize, Debug, JsonSchema)]
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

#[derive(Deserialize, Debug, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    #[default]
    All,
    GhostOnly,
    PlayerOnly,
}

#[derive(Deserialize, Debug, JsonSchema)]
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
