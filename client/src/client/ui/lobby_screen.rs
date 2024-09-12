use bevy::prelude::*;
use lightyear::{client::networking::NetworkingState, prelude::client::ClientCommands};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ScreenLobby;

#[derive(Component)]
pub struct ConnectButton;

pub fn lobby_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(350.0),
        height: Val::Px(125.0),
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

    // Root noded
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            ScreenLobby,
        ))
        // First column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Lobby",
                        TextStyle {
                            font: asset_server.load("grafitti.ttf"),
                            font_size: 80.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                border_color: BorderColor(Color::BLACK),
                                ..default()
                            },
                            ConnectButton, // Insert DuelButton here
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "JOIN LOBBY",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

// Responsible for simple connections
pub fn connect_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<ConnectButton>),
    >,
    mut text_query: Query<&mut Text>,
    network_state: Res<State<NetworkingState>>,
    mut commands: Commands,
) {
    // Thus button only contains one child text

    if let Ok((interaction, mut color, mut border_color, children)) =
        interaction_query.get_single_mut()
    {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match network_state.get() {
            NetworkingState::Disconnected => match *interaction {
                Interaction::Pressed => {
                    text.sections[0].value = "LETS DUEL!".to_string();
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = Color::srgb(255.0, 0.0, 0.0);
                    commands.connect_client();
                }
                Interaction::Hovered => {
                    text.sections[0].value = "DO IT ".to_string();
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    text.sections[0].value = "CONNECT TO SERVER".to_string();
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            },
            NetworkingState::Connecting => {
                text.sections[0].value = "Connecting".to_string();
            }
            NetworkingState::Connected => {
                text.sections[0].value = "Connected".to_string();
            }
        }
    }
}
