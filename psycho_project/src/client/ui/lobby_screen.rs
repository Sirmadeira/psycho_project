use crate::client::load_assets::Images;
use crate::client::rtt::{spawn_rtt_orbit_camera, RttImages};
use crate::client::MyAppState;
use crate::shared::protocol::lobby_structs::{SearchMatch, StartGame, StopSearch};
use crate::shared::protocol::player_structs::Channel1;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    color::palettes::basic::WHITE,
    input::mouse::{MouseScrollUnit, MouseWheel},
};
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera};
use lightyear::prelude::client::*;

// Plugin utilized to do all lobby related actions
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        // Debug
        app.register_type::<VisualToChange>();
        //Registering resource
        app.init_resource::<ToDisplayVisuals>();
        // Lobby systems
        app.add_systems(OnEnter(MyAppState::Lobby), lobby_screen);
        app.add_systems(
            Update,
            fill_rtt_ui_images.run_if(in_state(MyAppState::Lobby)),
        );
        app.add_systems(Update, search_button.run_if(in_state(MyAppState::Lobby)));
        app.add_systems(Update, change_button.run_if(in_state(MyAppState::Lobby)));
        app.add_systems(Update, scrolling_list.run_if(in_state(MyAppState::Lobby)));
        app.add_systems(Update, display_matches.run_if(in_state(MyAppState::Lobby)));
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Marker component for general lobby screen just despawn this guy and it is children when done
#[derive(Component)]
pub struct ScreenLobby;

//When clicked set server status to looking for match
#[derive(Component)]
struct SearchButton;

// A simple event that is sended when you click one of the visuasl to change buttons
#[derive(Resource, Reflect, Clone, Default)]
#[reflect(Resource)]
pub struct ToDisplayVisuals(pub VisualToChange);

// Tell me what exact visual the player wants to see
#[derive(Component, Debug, Clone, Reflect)]
pub enum VisualToChange {
    Head(Vec<ImageVisualInfo>),
    Torso(Vec<ImageVisualInfo>),
    Legs(Vec<ImageVisualInfo>),
}
impl Default for VisualToChange {
    fn default() -> Self {
        VisualToChange::Head(vec![
            ImageVisualInfo::new(
                "images/default.png".to_string(),
                "characters/parts/suit_head.glb".to_string(),
            ),
            ImageVisualInfo::new(
                "images/default.png".to_string(),
                "characters/parts/soldier_head.glb".to_string(),
            ),
        ])
    }
}
// This will ensure you only see a portion of the given images - TODO- MAKE IT IMAGES OF SCENES
impl VisualToChange {
    pub fn default_head() -> Self {
        VisualToChange::Head(vec![
            ImageVisualInfo::new(
                "images/default.png".to_string(),
                "characters/parts/suit_head.glb".to_string(),
            ),
            ImageVisualInfo::new(
                "images/shatur.png".to_string(),
                "characters/parts/soldier_head.glb".to_string(),
            ),
        ])
    }
    pub fn default_torso() -> Self {
        VisualToChange::Torso(vec![
            ImageVisualInfo::new(
                "images/default.png".to_string(),
                "characters/parts/scifi_torso.glb".to_string(),
            ),
            ImageVisualInfo::new(
                "images/default.png".to_string(),
                "characters/parts/soldier_torso.glb".to_string(),
            ),
        ])
    }
    pub fn default_legs() -> Self {
        VisualToChange::Legs(vec![
            ImageVisualInfo::new(
                "images/shatur.png".to_string(),
                "characters/parts/witch_legs.glb".to_string(),
            ),
            ImageVisualInfo::new(
                "images/default.png".to_string(),
                "characters/parts/soldier_legs.glb".to_string(),
            ),
        ])
    }
}

#[derive(Reflect, Debug, Clone)]
// Simple acessor utilized for having an easy way to acess string
pub struct ImageVisualInfo {
    //File path to image asset
    pub image: String,
    // File path to visual asset
    pub asset: String,
}

impl ImageVisualInfo {
    fn new(image: String, asset: String) -> Self {
        return Self {
            image: image,
            asset: asset,
        };
    }
}

//Placedholder on where to put our ui image and what ui image to put grab via file path()
#[derive(Component)]
struct RttPlaceholder(String);

// Made to tell me if player is searching for match in server, I avoided state here because I want the user
// To still be capable of sending messages to server
#[derive(Component)]
struct IsSearching;

// Scrolling list of available fights
#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn lobby_screen(asset_server: Res<AssetServer>, images: Res<Images>, mut commands: Commands) {
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

    let image_button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(200.0),
        border: UiRect::all(Val::Px(15.0)),
        margin: UiRect::all(Val::Px(10.0)),
        ..default()
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
                background_color: Color::srgb(0.10, 0.10, 0.10).into(),
                ..default()
            },
            ScreenLobby,
        ))
        .with_children(|parent| {
            // First column
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(33.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        // SIMPLE TITLE TEXT
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "SEARCH FOR MATCH",
                                TextStyle {
                                    font: asset_server.load("grafitti.ttf"),
                                    font_size: 45.0,
                                    ..default()
                                },
                            ));
                            // SEARCH FOR MATCH BUTTON
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        border_color: BorderColor(Color::BLACK),
                                        ..default()
                                    },
                                    SearchButton, // Insert DuelButton here
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "",
                                        button_text_style.clone(),
                                    ));
                                });
                        });
                });
            // SECOND COLUMN
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(33.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(20.0)), // Optional spacing around the row
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Change Head",
                                        TextStyle {
                                            font: asset_server.load("grafitti.ttf"),
                                            font_size: 40.,
                                            ..default()
                                        },
                                    ));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: image_button_style.clone(),
                                                border_color: BorderColor(Color::BLACK),
                                                ..default()
                                            },
                                            VisualToChange::default_head(),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(100.0), // Image width fills the button
                                                        height: Val::Percent(100.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                    images
                                                        .map
                                                        .get("images/default.png")
                                                        .expect("Default image to exist")
                                                        .clone(),
                                                ),
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Change body",
                                        TextStyle {
                                            font: asset_server.load("grafitti.ttf"),
                                            font_size: 40.,
                                            ..default()
                                        },
                                    ));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: image_button_style.clone(),
                                                border_color: BorderColor(Color::BLACK),
                                                ..default()
                                            },
                                            VisualToChange::default_torso(),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(100.0), // Image width fills the button
                                                        height: Val::Percent(100.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                    images
                                                        .map
                                                        .get("images/default.png")
                                                        .expect("Default image to exist")
                                                        .clone(),
                                                ),
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Change leg",
                                        TextStyle {
                                            font: asset_server.load("grafitti.ttf"),
                                            font_size: 40.,
                                            ..default()
                                        },
                                    ));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: image_button_style.clone(),
                                                border_color: BorderColor(Color::BLACK),
                                                ..default()
                                            },
                                            VisualToChange::default_legs(),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(100.0), // Image width fills the button
                                                        height: Val::Percent(100.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                    images
                                                        .map
                                                        .get("images/default.png")
                                                        .expect("Default image to exist")
                                                        .clone(),
                                                ),
                                            ));
                                        });
                                });
                        });

                    // Title for scrolling list
                    parent.spawn(TextBundle::from_section(
                        "WHO IS FIGHTING",
                        TextStyle {
                            font: asset_server.load("grafitti.ttf"),
                            font_size: 40.,
                            ..default()
                        },
                    ));
                    // LIST OF CURRENT LOBBIES - TODO MAKE PLAYER FIGHTING
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(75.),
                                overflow: Overflow::clip_y(),
                                border: UiRect::all(Val::Px(20.)),
                                ..default()
                            },
                            border_color: WHITE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Actual list
                            parent.spawn((
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
                            ));
                        });
                });
            // Third column
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(33.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "WHO ARE YOU?",
                        TextStyle {
                            font: asset_server.load("grafitti.ttf"),
                            font_size: 40.,
                            ..default()
                        },
                    ));
                    // RTT image with formating node
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(90.),
                                ..default()
                            },
                            ..default()
                        },
                        RttPlaceholder("Character".to_string()),
                    ));
                });
        });
}

fn search_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            Entity,
        ),
        (Changed<Interaction>, With<SearchButton>),
    >,
    is_searching_query: Query<Has<IsSearching>, With<SearchButton>>,
    mut connection_manager: ResMut<ConnectionManager>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
) {
    if let Ok((interaction, mut color, mut border_color, children, button_entity)) =
        interaction_query.get_single_mut()
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let is_searching = is_searching_query
            .get_single()
            .expect("Has statement to work as it should");

        // Interaction behavior logic
        if let Interaction::Pressed = *interaction {
            if is_searching {
                info!("Sending message to server to stop searching");
                let _ = connection_manager.send_message::<Channel1, StopSearch>(&mut StopSearch);
                commands.entity(button_entity).remove::<IsSearching>();
            } else {
                info!("Sending message to server to start searching");
                let _ = connection_manager.send_message::<Channel1, SearchMatch>(&mut SearchMatch);
                commands.entity(button_entity).insert(IsSearching);
            }
        }

        // Handle interaction directly
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = if is_searching {
                    "WHY DID YOU STOP NOO".to_string()
                } else {
                    "LETS GO".to_string()
                };
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::srgb(255.0, 0.0, 0.0);
            }
            Interaction::Hovered => {
                text.sections[0].value = if is_searching {
                    "CURRENTLY SEARCHING".to_string()
                } else {
                    "OH YEAH".to_string()
                };
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = if is_searching {
                    "CURRENTLY SEARCHING".to_string()
                } else {
                    "SEARCH FOR MATCH".to_string()
                };
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

// Send a message to server telling me player loadout
fn change_button(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &mut VisualToChange),
        (Changed<Interaction>, With<VisualToChange>),
    >,
    mut my_app_state: ResMut<NextState<MyAppState>>,
    lobby_screen: Query<Entity, With<ScreenLobby>>,
    mut send_visual: ResMut<ToDisplayVisuals>,
    mut commands: Commands,
) {
    for (interaction, mut border_color, visual_change) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = BorderColor(Color::WHITE);
                if let Ok(lobby_screen) = lobby_screen.get_single() {
                    info!("Despawning lobby scrren");
                    commands.entity(lobby_screen).despawn_recursive();

                    info!("Writing in resource what items to send to inv scrren");
                    *send_visual = ToDisplayVisuals(visual_change.clone());

                    info!("Setting new state");
                    my_app_state.set(MyAppState::Inventory);
                } else {
                    error!("COuldnt find lobby screen what in tarnation")
                }
            }
            Interaction::Hovered => {
                *border_color = BorderColor(Color::WHITE);
            }
            Interaction::None => {
                *border_color = BorderColor(Color::BLACK);
            }
        }
    }
}

// Responsible for scrolling up and down the matches list
fn scrolling_list(
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

// When a game starts update the list to other clients
fn display_matches(
    query_list: Query<Entity, With<ScrollingList>>,
    mut events: EventReader<MessageEvent<StartGame>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let scroll_parent = query_list
        .get_single()
        .expect("To have only one scrolling list");

    for event in events.read() {
        let lobby_id = event.message().lobby_id;

        let list_item = (
            TextBundle::from_section(
                format!("Lobby"),
                TextStyle {
                    font: asset_server.load("grafitti.ttf"),
                    ..default()
                },
            ),
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        );
        let mut child_entity = commands.spawn(list_item);
        child_entity.set_parent(scroll_parent);
        info!("Current lobbies displayed {}", lobby_id);
    }
}

// Grabs marker component
fn fill_rtt_ui_images(
    rtt_images: Res<RttImages>,
    mut rtt_placeholders: Query<(Entity, &RttPlaceholder), Added<RttPlaceholder>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut active_cam: ResMut<ActiveCameraData>,
    camera: Query<Entity, With<PanOrbitCamera>>,
    mut commands: Commands,
) {
    for (ui_image, placeholder) in rtt_placeholders.iter_mut() {
        if let Some(corresponding_image) = rtt_images.0.get(&placeholder.0) {
            if let Ok(cam) = camera.get_single() {
                warn!(
                    "You already spawned a camera entity {:?}so i am just gonna reuse it",
                    cam
                )
            } else {
                spawn_rtt_orbit_camera(
                    corresponding_image,
                    &windows,
                    &mut active_cam,
                    &mut commands,
                );
            }

            commands
                .entity(ui_image)
                .insert(UiImage::new(corresponding_image.handle.clone()));
        } else {
            warn!("Couldnt find the rtt for {}", placeholder.0);
        }
    }
}
