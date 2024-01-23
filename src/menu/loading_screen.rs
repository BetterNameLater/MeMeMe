use crate::player::Ghost;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

#[derive(Component)]
pub struct LoadingScreen;

pub fn loading_screen(mut commands: Commands) {
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
            LoadingScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Loading...",
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

pub fn stop_loading_screen(
    mut commands: Commands,
    mut loading_screen_query: Query<Entity, With<LoadingScreen>>,
) {
    let loading_screen = loading_screen_query.single();
    commands.entity(loading_screen).despawn_recursive();
}
