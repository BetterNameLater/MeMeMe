use crate::ldtk_json::{EntityInstance, GridPoint, LdtkJson, Level, ReferenceToAnEntityInstance};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::prelude::TypePath;
use maths::Vec2i;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct WorldRepr {
    pub levels: HashMap<String, MapRepr>,
}

impl WorldRepr {
    pub fn from_ldtk(world: LdtkJson) -> Self {
        Self {
            levels: world
                .levels
                .into_iter()
                .map(|level| (level.identifier.clone(), MapRepr::from_ldtk(level)))
                .collect(),
        }
    }
}

#[derive(Asset, TypePath, Deserialize, Debug, JsonSchema)]
pub struct MapRepr {
    map: Vec<String>,
    pub objects: HashMap<String, ObjectRepr>,
    pub start: Vec2i,
    pub goal: Option<Vec2i>,
}

impl MapRepr {
    pub fn map(&self) -> Vec<Vec<BackgroundType>> {
        self.map
            .iter()
            .map(|line| {
                line.chars()
                    .map(|background_type| {
                        background_type
                            .try_into()
                            .expect("background type not recognized")
                    })
                    .collect()
            })
            .collect()
    }

    pub fn from_ldtk(level: Level) -> Self {
        let (width, height) = ((level.px_wid / 16), (level.px_hei / 16));
        let layer_instances = level.layer_instances.clone().unwrap();

        let start = {
            let [x, y] = layer_instances[1]
                .entity_instances
                .iter()
                .find(|entity| entity.identifier == "start")
                .unwrap()
                .grid[0..2]
            else {
                panic!();
            };
            let y = height - y - 1;
            Vec2i::from((x, y))
        };

        let goal = {
            if let Some(entity) = layer_instances[1]
                .entity_instances
                .iter()
                .find(|entity| entity.identifier == "goal")
            {
                let [x, y] = entity.grid[0..2] else { panic!() };
                let y = height - y - 1;
                Some(Vec2i::from((x, y)))
            } else {
                None
            }
        };

        Self {
            map: layer_instances[2]
                .int_grid_csv
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .chunks(width as usize)
                .map(|line| line.concat())
                .collect(),
            objects: layer_instances[0]
                .entity_instances
                .clone()
                .into_iter()
                .map(|entity| (entity.iid.clone(), ObjectRepr::from_ldtk(entity, height)))
                .collect(),
            start,
            goal,
        }
    }
}

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
    type Asset = WorldRepr;
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
        match serde_json::from_slice::<LdtkJson>(&bytes) {
            Ok(r) => Ok(WorldRepr::from_ldtk(r)),
            Err(e) => Err(e.into()),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

#[derive(Deserialize, Debug, PartialEq, JsonSchema)]
pub enum BackgroundType {
    Void,
    Floor,
    Wall,
    Start,
    End,
}

impl TryFrom<char> for BackgroundType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' | '0' => Ok(BackgroundType::Void),
            '2' | '+' | '-' => Ok(BackgroundType::Floor),
            '1' => Ok(BackgroundType::Wall),
            _ => Err(()),
        }
    }
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

impl ObjectRepr {
    pub fn from_ldtk(entity: EntityInstance, height: i64) -> Self {
        let position = {
            let [x, y] = entity.grid[0..2] else {
                panic!();
            };
            let y = height - y - 1;
            Vec2i::from((x, y))
        };

        Self {
            position,
            object_type: match entity.identifier.as_str() {
                "pressure_plate" => ObjectType::PressurePlate,
                "level_teleporter" => ObjectType::LevelTeleporter {
                    destination: get_string_field(&entity, "destination"),
                },
                "door" => ObjectType::Door,
                "teleporter" => ObjectType::Teleporter {
                    destination: {
                        let destination = get_field_ref(&entity, "destination").unwrap();
                        let destination: GridPoint =
                            serde_json::from_value(destination.clone()).unwrap();
                        Vec2i::from((destination.cx, height - destination.cy - 1))
                    },
                },
                "lever" => ObjectType::Lever,
                "button" => ObjectType::Button,
                _ => panic!("what is a \"{}\" ?", entity.identifier),
            },
            interaction_type: InteractionType::All,
            single_use: get_single_use(&entity),
            killing: false,
            depends_on: get_depends_on(&entity),
            start_timer: None,
        }
    }
}

fn get_field_ref<'a>(entity: &'a EntityInstance, name: &str) -> Option<&'a Value> {
    let Some(field) = &entity.field_instances.iter().find(|f| f.identifier == name) else {
        return None;
    };
    Some(field.value.as_ref().unwrap())
}

fn get_single_use(entity: &EntityInstance) -> bool {
    get_field_ref(&entity, "usage_type")
        .unwrap_or(&json!("nope"))
        .as_str()
        == Some("single_use")
}

fn get_string_field(entity: &EntityInstance, name: &str) -> String {
    get_field_ref(entity, name)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn get_depends_on(entity: &EntityInstance) -> HashMap<String, bool> {
    if let Some(props) = entity
        .field_instances
        .iter()
        .find(|f| f.identifier == "depends_on" || f.identifier == "depends_off")
    {
        props
            .value
            .as_ref()
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                (
                    serde_json::from_value::<ReferenceToAnEntityInstance>(v.clone())
                        .unwrap()
                        .entity_iid,
                    props.identifier == "depends_on",
                )
            })
            .collect()
    } else {
        HashMap::new()
    }
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
