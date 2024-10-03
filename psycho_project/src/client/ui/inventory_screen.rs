//! Responsible for displaying in little squares the current items available to client
use bevy::prelude::*;

use crate::client::load_assets::Images;
use crate::client::MyAppState;

use super::lobby_screen::{ToDisplayVisuals, VisualToChange};
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(MyAppState::Lobby),
            (inventory_screen, display_selected_visuals).chain(),
        );
    }
}

// Simple marker component that gives me all my images made for hover logic
#[derive(Component)]
struct ImageButton;

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
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::srgb(0.10, 0.10, 0.10).into(),
            ..default()
        },))
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
                                .spawn((ButtonBundle {
                                    style: button_style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "RETURN NOW",
                                        button_text_style.clone(),
                                    ));
                                });
                        });
                })
                //Node that spawns sub childrend
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
                "Change Visual", // Title can be dynamic if needed
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
                    ImageButton,
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

// System responsible for listening to change head event and such events
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
                        visual,
                        &image_button_style,
                        &asset_server,
                        &images,
                    ));
                }
            }
            VisualToChange::Torso(visuals) => {
                info!("Creating children of organizing node");
                for visual in visuals.iter() {
                    childs.push(spawn_image_button(
                        &mut commands,
                        visual,
                        &image_button_style,
                        &asset_server,
                        &images,
                    ));
                }
            }
            VisualToChange::Legs(visuals) => {
                info!("Creating children of organizing node");
                for visual in visuals.iter() {
                    childs.push(spawn_image_button(
                        &mut commands,
                        visual,
                        &image_button_style,
                        &asset_server,
                        &images,
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
