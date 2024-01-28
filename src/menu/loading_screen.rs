use bevy::prelude::*;

#[derive(Component)]
pub struct MessageScreen;

fn message_screen(mut commands: Commands, message: &str) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            MessageScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    message,
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                Label,
            ));
        });
}

pub fn loading_screen(commands: Commands) {
    debug!("Load loading screen");
    message_screen(commands, "Loading...")
}

pub fn error_screen(commands: Commands) {
    debug!("Load error screen");
    message_screen(commands, "An error occurred")
}

pub fn unload_message_screen(
    mut commands: Commands,
    loading_screen_query: Query<Entity, With<MessageScreen>>,
) {
    debug!("Unload message screen");
    let loading_screen = loading_screen_query.single();
    commands.entity(loading_screen).despawn_recursive();
}
