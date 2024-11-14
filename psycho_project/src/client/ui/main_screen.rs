use crate::shared::protocol::CommonChannel;
use crate::{client::MyAppState, shared::protocol::lobby_structs::EnterLobby};
use bevy::prelude::*;
use lightyear::client::connection::ConnectionManager;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::MainMenu), menu_screen);
        app.add_systems(Update, start_button.run_if(in_state(MyAppState::MainMenu)));
        app.add_systems(Update, exit_button.run_if(in_state(MyAppState::MainMenu)));
    }
}

#[derive(Component)]

pub struct ScreenMainMenu;

// Marker component for the start button
#[derive(Component)]
struct StartButton;

// Marker component for the exit button
#[derive(Component)]
struct ExitButton;

fn menu_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(75.0),
        border: UiRect::all(Val::Px(15.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("grafitti.ttf"),
        font_size: 40.0,
        color: Color::srgb(0.9, 0.9, 0.9),
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
                background_color: Color::srgb(0.10, 0.10, 0.10).into(),
                ..default()
            },
            ScreenMainMenu,
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
                            color: Color::srgb(0.9, 0.9, 0.9),
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
                            StartButton, // Insert DuelButton here
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "LETS DUEL",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((ButtonBundle {
                            style: button_style.clone(),
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "SETTINGS",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ExitButton,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("QUIT", button_text_style.clone()));
                        });
                });
        });
}

/// Button responsible for entering pre existing lobby
fn start_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut connection_manager: ResMut<ConnectionManager>,
    mut text_query: Query<&mut Text>,
) {
    if let Ok((interaction, mut color, mut border_color, children)) =
        interaction_query.get_single_mut()
    {
        let mut text = text_query.get_mut(children[0]).unwrap();

        // Handle interaction directly
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "LETS F GO".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::srgb(255.0, 0.0, 0.0);
                let _ =
                    connection_manager.send_message::<CommonChannel, EnterLobby>(&mut EnterLobby);
            }
            Interaction::Hovered => {
                text.sections[0].value = "FIGHT".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "FIGHT".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

/// Buttons responsible for leaving app
fn exit_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        // Grabs entity text button
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "GOODBYE COWBOY".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::srgb(255.0, 0.0, 0.0);
                exit.send(AppExit::Success);
            }
            Interaction::Hovered => {
                text.sections[0].value = "I LOVE YOU DONT ".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "EXIT".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
