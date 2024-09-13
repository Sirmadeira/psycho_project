use crate::shared::protocol::lobby_structs::Lobbies;
use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::utils::HashMap;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ScreenLobby;

#[derive(Component)]
pub struct ConnectButton;

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

#[derive(Resource, Default, Debug)]
pub(crate) struct LobbyTable {
    clients: HashMap<ClientId, bool>,
}

impl LobbyTable {
    /// Find who will be the host of the game. If no client is host; the server will be the host.
    pub(crate) fn get_host(&self) -> Option<ClientId> {
        self.clients
            .iter()
            .find_map(|(client_id, is_host)| if *is_host { Some(*client_id) } else { None })
    }
}

pub fn lobby_screen(mut commands: Commands, asset_server: Res<AssetServer>, lobbies: Res<Lobbies>) {
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: Color::srgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // Check if lobby exists
                                    for (lobby_id, lobby) in lobbies.lobbies.iter().enumerate() {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("Item {lobby_id}"),
                                                TextStyle {
                                                    font: asset_server.load("grafitti.ttf"),
                                                    ..default()
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
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

pub fn scrolling_list(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
