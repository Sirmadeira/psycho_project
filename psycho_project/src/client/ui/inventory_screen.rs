//! Responsible for displaying in little squares the current items available to client
use crate::shared::protocol::player_structs::Channel1;
use bevy::prelude::*;
use lightyear::client::connection::ConnectionManager;

use crate::client::MyAppState;
use crate::client::{essentials::EasyClient, load_assets::Images};
use crate::shared::protocol::player_structs::{PlayerBundleMap, SaveVisual};

use super::pause_screen::{ToDisplayVisuals, VisualToChange};
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<ChangeChar>();
        //Debuging
        app.register_type::<ChangeChar>();
        app.register_type::<PartToChange>();

        app.add_systems(
            OnEnter(MyAppState::Inventory),
            (inventory_screen, display_selected_visuals).chain(),
        );
        app.add_systems(
            Update,
            return_button.run_if(in_state(MyAppState::Inventory)),
        );
        app.add_systems(
            Update,
            assets_buttons.run_if(in_state(MyAppState::Inventory)),
        );
    }
}

// Marker componet utilized to easily despawn entire inventory screen
#[derive(Component)]
pub struct ScreenInventory;

// AN event message sent by server to client that tells me that player can customize his character
#[derive(Event, Reflect)]
pub struct ChangeChar(pub PartToChange);

// Tell me the parts to change when grabing char customizer resource
#[derive(Reflect)]
pub struct PartToChange {
    // File path to old part
    pub old_part: String,
    // File Path to new part
    pub new_part: String,
}

#[derive(Component)]
struct ReturnButton;

// Component utilized to store the given asset that is correlated to that image
#[derive(Component)]
struct AssetButton(String);

// Simple marker tell me what node to insert children in
#[derive(Component)]
struct OrganizingNode;

fn inventory_screen(asset_server: Res<AssetServer>, mut commands: Commands) {
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

    info!("Spawning inventory screen");
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
            ScreenInventory,
        ))
        // First column
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(15.0),
                        height: Val::Percent(40.0),
                        ..default()
                    },
                    ..default()
                })
                // RETURN BUTTON
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_self: AlignSelf::FlexStart,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        border_color: BorderColor(Color::BLACK),
                                        ..default()
                                    },
                                    ReturnButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "RETURN NOW",
                                        button_text_style.clone(),
                                    ));
                                });
                        });
                })
                //Node that spawns sub visuals
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(20.0)), // Optional spacing around the row
                                ..default()
                            },
                            ..default()
                        },
                        OrganizingNode,
                    ));
                });
        });
}

// Helper function avoids a lot of sub code
fn spawn_image_button(
    commands: &mut Commands,
    image_path: &str,
    image_button_style: &Style,
    asset_server: &AssetServer,
    images: &Res<Images>,
    assets: String,
) -> Entity {
    let button_entity = commands
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
                format!("{}", assets),
                TextStyle {
                    font: asset_server.load("grafitti.ttf"),
                    font_size: 40.,
                    ..default()
                },
            ));

            // Spawn the button with the image
            parent
                .spawn((
                    ButtonBundle {
                        style: image_button_style.clone(),
                        border_color: BorderColor(Color::BLACK),
                        ..default()
                    },
                    AssetButton(assets),
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
                                .get(image_path)
                                .expect(&format!("Image '{}' to exist", image_path))
                                .clone(),
                        ),
                    ));
                });
        })
        .id();
    return button_entity;
}

// System that spawns the ui buttons responsible for displaying an image, with an acessor to an asset path
fn display_selected_visuals(
    organizing_node: Query<Entity, With<OrganizingNode>>,
    asset_server: Res<AssetServer>,
    images: Res<Images>,
    display_visuals: Res<ToDisplayVisuals>,
    mut commands: Commands,
) {
    if let Ok(node) = organizing_node.get_single() {
        info!("Found node lets spawn the images ");
        let image_button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(15.0)),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        };
        let mut childs = Vec::default();
        let what_to_display = display_visuals.0.clone();
        match what_to_display {
            VisualToChange::Head(visuals) => {
                info!("Creating children of organizing node");
                for visual in visuals.iter() {
                    childs.push(spawn_image_button(
                        &mut commands,
                        &visual.image,
                        &image_button_style,
                        &asset_server,
                        &images,
                        visual.asset.clone(),
                    ));
                }
            }
            VisualToChange::Torso(visuals) => {
                info!("Creating children of organizing node");
                for visual in visuals.iter() {
                    childs.push(spawn_image_button(
                        &mut commands,
                        &visual.image,
                        &image_button_style,
                        &asset_server,
                        &images,
                        visual.asset.clone(),
                    ));
                }
            }
            VisualToChange::Legs(visuals) => {
                info!("Creating children of organizing node");
                for visual in visuals.iter() {
                    childs.push(spawn_image_button(
                        &mut commands,
                        &visual.image,
                        &image_button_style,
                        &asset_server,
                        &images,
                        visual.asset.clone(),
                    ));
                }
            }
        }
        info!("Making image buttons child of organizing node");
        for child in childs.iter() {
            commands.entity(child.clone()).set_parent(node);
        }
    }
}

fn return_button(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<ReturnButton>),
    >,
    inv_screen: Query<Entity, With<ScreenInventory>>,
    mut next_state: ResMut<NextState<MyAppState>>,
    mut commands: Commands,
) {
    if let Ok((interaction, mut border_color)) = interaction_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(inv_screen) = inv_screen.get_single() {
                    info!("Despawning inventory returning to lobby");
                    next_state.set(MyAppState::Pause);
                    commands.entity(inv_screen).despawn_recursive();
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

fn assets_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &AssetButton),
        (Changed<Interaction>, With<AssetButton>),
    >,
    mut change_char: EventWriter<ChangeChar>,
    client_id: Res<EasyClient>,
    player_bundle_map: Res<PlayerBundleMap>,
    to_display_visuals: Res<ToDisplayVisuals>,
    mut connection_manager: ResMut<ConnectionManager>,
) {
    let visuals_displayed = to_display_visuals.0.clone();

    for (interaction, mut border_color, asset_button) in interaction_query.iter_mut() {
        let asset_path = asset_button.0.clone();
        match *interaction {
            Interaction::Pressed => {
                let client_id = client_id.0;
                info!("Grabbing client {}", client_id);

                if let Some(player_bundle_map) = player_bundle_map.0.get(&client_id) {
                    info!("Grabbing player current visuals");

                    let mut player_visuals = player_bundle_map.visuals.clone();

                    match visuals_displayed {
                        VisualToChange::Head(_) => {
                            info!("Visual to adjust was in head");

                            info!("Grabbing old part {}", player_visuals.head.clone());
                            info!("Grabbing new part {}", asset_path.clone());
                            
                            change_char.send(ChangeChar(PartToChange {
                                old_part: player_visuals.head.clone(),
                                new_part: asset_path.clone(),
                            }));

                            info!("Adjusting cloned visual to send to replicated resource later");
                            player_visuals.head = asset_path.clone();
                        }
                        VisualToChange::Torso(_) => {
                            info!("Grabbing old part {}", player_visuals.torso.clone());
                            info!("Grabbing new part {}", asset_path.clone());
                            change_char.send(ChangeChar(PartToChange {
                                old_part: player_visuals.torso.clone(),
                                new_part: asset_path.clone(),
                            }));

                            info!("Adjusting cloned visual to send to resource later");
                            player_visuals.torso = asset_path.clone();
                        }
                        VisualToChange::Legs(_) => {
                            info!("Grabbing old part {}", player_visuals.legs.clone());
                            info!("Grabbing new part {}", asset_path.clone());
                            change_char.send(ChangeChar(PartToChange {
                                old_part: player_visuals.legs.clone(),
                                new_part: asset_path.clone(),
                            }));

                            info!("Adjusting cloned visual to send to resource later");
                            player_visuals.legs = asset_path.clone();
                        }
                    }
                    info!("Sending message to server to adjust visual resource");
                    let _ = connection_manager.send_message::<Channel1, SaveVisual>(
                        &mut SaveVisual(player_visuals.clone()),
                    );
                } else {
                    error!("Couldnt find you in server my man cant adjust your visuals")
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
