use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::{
    constants::{Colors, RADIUS},
    cursor::Cursor,
    gate::{GateBundle, GateType, MovingGate},
    node::NodeSpawner,
};

pub struct UiBuilder;

impl Plugin for UiBuilder {
    fn build(&self, app: &mut App) {
        app.add_system(interact_gate_ui)
            .add_startup_system(create_gate_ui)
            .add_startup_system(create_input_ui)
            .add_system(align_input_nodes)
            .add_system(interact_remove_input_nodes)
            .add_system(interact_add_input_nodes);
    }
}

fn text_builder(text: &str, asset_server: &Res<AssetServer>) -> impl Bundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("FiraCode.ttf"),
            color: Color::WHITE,
            font_size: 24.0,
        },
    )
}

#[derive(Component)]
struct GateButton(GateType);

fn create_gate_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Auto, Val::Px(40.0)),
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::horizontal(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(75.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                size: Size::new(Val::Percent(100.0), Val::Auto),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Colors::UI_BG.into(),
            ..default()
        })
        .with_children(|c| {
            use GateType::*;
            let kinds = [And, Or, Xor, Not];

            for kind in kinds {
                let button_str = kind.as_str();
                c.spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: Colors::OFF.into(),
                        ..default()
                    },
                    GateButton(kind),
                ))
                .with_children(|c| {
                    c.spawn(text_builder(button_str, &asset_server));
                });
            }
        });
}

#[allow(clippy::type_complexity)]
fn interact_gate_ui(
    mut query: Query<(&Interaction, &mut BackgroundColor, &GateButton), Changed<Interaction>>,
    mut moving_gate: ResMut<MovingGate>,
    cursor: Res<Cursor>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color, GateButton(kind)) in &mut query {
        match *interaction {
            Interaction::None => *color = Colors::OFF.into(),
            Interaction::Hovered => *color = Colors::highlighted(false).into(),
            Interaction::Clicked => {
                *color = Colors::ON.into();

                let gate = GateBundle::new(&asset_server, (*kind).clone(), Vec2::splat(120.0))
                    .pos(cursor.0);
                let gate = gate.spawn(&mut commands).id();

                moving_gate.0 = Some((gate, Vec2::ZERO));
            }
        }
    }
}

#[derive(Component)]
struct RemoveInputRootMarker;

#[derive(Component)]
pub struct AddInputMarker;

#[derive(Component)]
pub struct RemoveInputMarker(Entity);

#[derive(Component)]
pub struct InputNodeMarker;

lazy_static! {
    static ref INPUT_BUTTON_STYLE: Style = Style {
        size: Size::new(Val::Px(RADIUS*2.0), Val::Px(RADIUS*2.0)),
        margin: UiRect::all(Val::Px(10.0)),
        ..default()
    };
}

fn create_input_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::left(Val::Px(0.0)),
                size: Size::new(Val::Px(75.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Colors::UI_BG.into(),
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                RemoveInputRootMarker
            ));

            c.spawn((
                ButtonBundle {
                    style: INPUT_BUTTON_STYLE.clone(),
                    image: asset_server.load("add_input_node.png").into(),
                    ..default()
                },
                AddInputMarker
            ));
        });
}

/// This needed a bit of hackery to translate screen space coordinates into world space coordinates,
/// as it was not possible to simply put the nodes as childs of the ui
fn align_input_nodes(
    buttons: Query<(&GlobalTransform, &RemoveInputMarker)>,
    mut nodes: Query<&mut Transform, With<InputNodeMarker>>,
    windows: Res<Windows>,
    camera_transform: Query<&Transform, (With<Camera>, Without<InputNodeMarker>)>,
) {
    let wnd = windows.get_primary().unwrap();
    let camera_transform = camera_transform.get_single().unwrap();

    for (transform, &RemoveInputMarker(node)) in buttons.iter() {
        let position = transform.translation();
        let norm = Vec3::new(
            position.x - wnd.width() / 2.0,
            wnd.height() / 2.0 - position.y,
            0.0,
        );

        let world = *camera_transform * norm;

        let mut node_transform = nodes.get_mut(node).unwrap();

        let new_transform = world.truncate() + Vec2::new(75.0, 0.0);

        node_transform.translation.x = new_transform.x;
        node_transform.translation.y = new_transform.y;
    }
}

fn interact_remove_input_nodes(
    mut commands: Commands,
    mut buttons: Query<(Entity, &mut BackgroundColor, &Interaction, &RemoveInputMarker), Changed<Interaction>>
) {
    for (entity, mut color, interaction, &RemoveInputMarker(node)) in &mut buttons {
        match interaction {
            Interaction::None => color.0 = Colors::OFF,
            Interaction::Hovered => color.0 = Colors::highlighted(false),
            Interaction::Clicked => {
                commands.get_entity(node).unwrap().despawn_recursive();
                commands.get_entity(entity).unwrap().despawn_recursive();
            }
        }
    }
}

fn interact_add_input_nodes(
    mut commands: Commands,
    mut buttons: Query<(&mut BackgroundColor, &Interaction), (Changed<Interaction>, With<AddInputMarker>)>,
    root: Query<Entity, With<RemoveInputRootMarker>>,
    asset_server: Res<AssetServer>
) {
    let root = root.get_single().unwrap();

    for (mut color, interaction) in &mut buttons {
        match interaction {
            Interaction::None => color.0 = Colors::OFF,
            Interaction::Hovered => color.0 = Colors::highlighted(false),
            Interaction::Clicked => {
                let node = commands.spawn((NodeSpawner::new(), InputNodeMarker)).id();
                let remove_button = commands.spawn((
                    ButtonBundle {
                        style: INPUT_BUTTON_STYLE.clone(),
                        image: asset_server.load("remove_input_node.png").into(),
                        ..default()
                    },
                    RemoveInputMarker(node)
                )).id();
                commands.get_entity(root).unwrap().add_child(remove_button);
            }
        }
    }
}
