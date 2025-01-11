mod ghost_actions;
mod movement;
mod rewind;

mod utils {
    pub use crate::constantes::*;
    use crate::level::components::level_tag::LevelTag;
    pub use crate::level::ressources::level_informations::LevelInformations;
    pub use crate::math::vec2i::Vec2i;
    pub use std::time::Duration;

    use crate::map::{Map, ObjectMap};
    use crate::{
        items::events::{OnEnterEvent, OnExitEvent},
        player::{components::player::Player, GhostActions},
    };
    use bevy::prelude::KeyCode;
    use bevy::prelude::*;

    pub const PLAYER_ORIGIN: Transform = Transform::from_xyz(0., 0., PLAYER_Z);
    pub const SECOND: Duration = Duration::from_secs(1);

    pub fn base_init() -> App {
        let mut app = App::new();

        app.init_resource::<Time>()
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

        assert_eq!(player(&mut app).actions.len(), 0);
        assert_eq!(
            player_transform(&mut app),
            &Vec2i::default().to_transform(PLAYER_Z)
        );
        app
    }

    pub fn player(app: &mut App) -> &Player {
        query_single!(app, Player)
    }

    pub fn player_transform(app: &mut App) -> &Transform {
        app.world_mut()
            .query_filtered::<&Transform, With<Player>>()
            .single(app.world())
    }

    macro_rules! query_single {
        ($app:ident, $t:tt ) => {
            $app.world_mut().query::<&$t>().single($app.world())
        };
    }

    macro_rules! query_single_mut {
        ($app:ident, $t:tt ) => {
            $app.world_mut()
                .query::<&mut $t>()
                .single_mut($app.world_mut())
        };
    }

    macro_rules! resource {
        ($app:ident, $t:tt) => {
            $app.world().resource::<$t>()
        };
    }

    macro_rules! resource_mut {
        ($app:ident, $t:tt) => {
            $app.world_mut().resource_mut::<$t>()
        };
    }

    macro_rules! advance_to {
        ($app:ident, $duration:expr) => {{
            use bevy::time::Time as MacroSecretTimeType;
            resource_mut!($app, MacroSecretTimeType).advance_to($duration);
        }};
    }

    macro_rules! advance_by {
        ($app:ident, $duration:expr) => {{
            use bevy::time::Time as MacroSecretTimeType;
            resource_mut!($app, MacroSecretTimeType).advance_by($duration);
        }};
    }

    macro_rules! press_key_and_update {
        ($app:ident, $key:expr) => {{
            use bevy::input::ButtonInput as MacroSecretButtonInput;
            use bevy::prelude::KeyCode as MacroSecretKeyCode;
            type Input = MacroSecretButtonInput<MacroSecretKeyCode>;

            resource_mut!($app, Input).press($key);
            $app.update();
            resource_mut!($app, Input).clear();
        }};
    }

    pub(crate) use advance_by;
    pub(crate) use advance_to;
    pub(crate) use press_key_and_update;
    pub(crate) use query_single;
    pub(crate) use query_single_mut;
    pub(crate) use resource;
    pub(crate) use resource_mut;
}

/// Test if the time logic for tests in working
mod time {
    use super::utils::*;
    use bevy::time::Time;

    #[test]
    fn start_at_0() {
        let app = base_init();
        assert_eq!(resource!(app, Time).elapsed_secs(), 0.);
    }

    #[test]
    fn update_do_not_advance() {
        let mut app = base_init();
        assert_eq!(resource!(app, Time).elapsed_secs(), 0.);

        app.update();
        app.update();
        app.update();
        app.update();

        assert_eq!(resource!(app, Time).elapsed_secs(), 0.);
    }

    #[test]
    fn advance_to_() {
        let mut app = base_init();

        advance_to!(app, Duration::from_secs(6));
        assert_eq!(resource!(app, Time).elapsed_secs(), 6.);

        advance_to!(app, Duration::from_secs(12));
        assert_eq!(resource!(app, Time).elapsed_secs(), 12.);
    }

    #[test]
    fn advance_by_() {
        let mut app = base_init();

        advance_by!(app, Duration::from_secs(6));
        assert_eq!(resource!(app, Time).elapsed_secs(), 6.);

        advance_by!(app, Duration::from_secs(6));
        assert_eq!(resource!(app, Time).elapsed_secs(), 12.);
    }

    #[test]
    #[should_panic]
    fn advance_to_past_panic() {
        let mut app = base_init();

        advance_to!(app, Duration::from_secs(6));
        assert_eq!(resource!(app, Time).elapsed_secs(), 6.);

        // this should panic
        advance_to!(app, Duration::from_secs(0));
    }
}
