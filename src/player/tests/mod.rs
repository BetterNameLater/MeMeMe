mod movement;

mod utils {
    pub use crate::constantes::*;
    use crate::level::components::level_tag::LevelTag;
    pub use crate::math::vec2i::Vec2i;

    use crate::map::{Map, ObjectMap};
    use crate::{
        items::events::{OnEnterEvent, OnExitEvent},
        level::ressources::level_informations::LevelInformations,
        player::{components::player::Player, GhostActions},
    };
    use bevy::prelude::KeyCode;
    use bevy::prelude::*;
    use bevy::time::TimePlugin;

    pub const PLAYER_ORIGIN: Transform = Transform::from_xyz(0., 0., PLAYER_Z);

    pub fn base_init() -> App {
        let mut app = App::new();

        app.add_plugins(TimePlugin)
            .insert_resource(ButtonInput::<KeyCode>::default())
            .insert_resource(GhostActions::default())
            .insert_resource(LevelInformations::default())
            .add_event::<OnEnterEvent>()
            .add_event::<OnExitEvent>();

        // spawn Player, Map, LevelTag
        app.world_mut()
            .commands()
            .spawn((Map::default(), ObjectMap));
        app.world_mut().commands().spawn((Map::default(), LevelTag));
        Player::spawn_player(&mut app.world_mut().commands(), Vec2i::default());
        app.update();

        assert_eq!(get_player(&mut app).actions.len(), 0);
        assert_eq!(
            get_player_pos(&mut app),
            &Vec2i::default().to_transform(PLAYER_Z)
        );
        app
    }

    pub fn query_single<T: Component>(app: &mut App) -> &T {
        app.world_mut().query::<&T>().single(app.world())
    }

    pub fn get_player(app: &mut App) -> &Player {
        query_single(app)
    }

    pub fn get_player_pos(app: &mut App) -> &Transform {
        app.world_mut()
            .query_filtered::<&Transform, With<Player>>()
            .single(app.world())
    }

    pub fn press_key_and_update(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
    }
}
