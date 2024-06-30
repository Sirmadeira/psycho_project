use bevy::prelude::*;

use crate::MyAppState;

use self::{debug_ui::*,lib::*};

mod debug_ui;
mod lib;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_begin_camera);
        app.add_systems(OnEnter(MyAppState::MainMenu), (spawn_entities, spawn_debug));
        app.add_systems(Update, start_button);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);




// This occurs first that is why is separated
fn spawn_begin_camera(mut commands : Commands){
    commands.spawn(Camera2dBundle::default()).insert(UiCamera);
}


fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },OnMainMenu))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Px(10.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("grafitti.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn start_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut ui_camera: Query<Entity,With<UiCamera>>,
    mut on_main_menu: Query<Entity,With<OnMainMenu>>,
    mut my_app_state: ResMut<NextState<MyAppState>>,
    mut commands: Commands
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        // Grabs entity text button
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                my_app_state.set(MyAppState::InGame);
                let cam = ui_camera.get_single_mut().unwrap();
                let menu = on_main_menu.get_single_mut().unwrap();
                commands.entity(cam).despawn();
                commands.entity(menu).despawn_descendants().despawn();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
