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

    macro_rules! resource {
        ($t:tt, $app:ident ) => {
            $app.world().resource::<$t>()
        };
    }
    pub(crate) use resource;

    macro_rules! resource_mut {
        ($t:tt, $app:ident ) => {
            $app.world_mut().resource_mut::<$t>()
        };
    }
    pub(crate) use resource_mut;

    pub fn advance_to(app: &mut App, duration: Duration) {
        resource_mut!(Time, app).advance_to(duration);
    }

    pub fn press_key_and_update(app: &mut App, key: KeyCode) {
        type Input = ButtonInput<KeyCode>;

        resource_mut!(Input, app).press(key);
        app.update();
        resource_mut!(Input, app).clear();
    }
}

/// Test if the time logic for tests in working
mod time {
    use super::utils::*;
    use bevy::time::Time;

    #[test]
    fn start_at_0() {
        let app = base_init();
        assert_eq!(resource!(Time, app).elapsed_secs(), 0.);
    }

    #[test]
    fn update_do_not_advance() {
        let mut app = base_init();
        assert_eq!(resource!(Time, app).elapsed_secs(), 0.);

        app.update();
        app.update();
        app.update();
        app.update();

        assert_eq!(resource!(Time, app).elapsed_secs(), 0.);
    }

    #[test]
    fn advance_to_() {
        let mut app = base_init();

        advance_to(&mut app, Duration::from_secs(6));
        assert_eq!(resource!(Time, app).elapsed_secs(), 6.);

        advance_to(&mut app, Duration::from_secs(12));
        assert_eq!(resource!(Time, app).elapsed_secs(), 12.);
    }

    #[test]
    #[should_panic]
    fn advance_to_past_panic() {
        let mut app = base_init();

        advance_to(&mut app, Duration::from_secs(6));
        assert_eq!(resource!(Time, app).elapsed_secs(), 6.);

        // this should panic
        advance_to(&mut app, Duration::from_secs(0));
    }
}
