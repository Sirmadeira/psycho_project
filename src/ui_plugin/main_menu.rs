use crate::ui_plugin::{OnMainMenu, UiCamera};
use crate::MyAppState;
use bevy::prelude::*;

use super::DuelButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(75.0),
        border: UiRect::all(Val::Px(10.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("grafitti.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "PSYCHO PROJECT",
                        TextStyle {
                            font: asset_server.load("grafitti.ttf"),
                            font_size: 80.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                    
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            DuelButton, // Insert DuelButton here
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "LETS DUEL",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "SETTINGS",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "QUIT",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}


pub fn start_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<DuelButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut ui_camera: Query<Entity, With<UiCamera>>,
    mut on_main_menu: Query<Entity, With<OnMainMenu>>,
    mut my_app_state: ResMut<NextState<MyAppState>>,
    mut commands: Commands,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        // Grabs entity text button
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "LETS DUEL!".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                my_app_state.set(MyAppState::InGame);
                let cam = ui_camera.get_single_mut().unwrap();
                let menu = on_main_menu.get_single_mut().unwrap();
                commands.entity(cam).despawn();
                commands.entity(menu).despawn_descendants().despawn();
            }
            Interaction::Hovered => {
                text.sections[0].value = "IDIOT!".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "GET STARTED".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
