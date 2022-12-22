use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    constants::{Colors, Depth},
    gate::{GateType, MovingGate, GateBundle}, cursor::Cursor,
};

pub struct UiBuilder;

impl Plugin for UiBuilder {
    fn build(&self, app: &mut App) {
        app.add_system(interact_gate_ui)
            .add_startup_system(create_gate_ui)
            .add_startup_system(create_input_ui);
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
                    left: Val::Percent(15.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                size: Size::new(Val::Auto, Val::Auto),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
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
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &GateButton),
        Changed<Interaction>,
    >,
    mut moving_gate: ResMut<MovingGate>,
    cursor: Res<Cursor>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (interaction, mut color, GateButton( kind )) in &mut query {
        match *interaction {
            Interaction::None => *color = Colors::OFF.into(),
            Interaction::Hovered => *color = Colors::highlighted(false).into(),
            Interaction::Clicked => {
                *color = Colors::ON.into();

                let gate = GateBundle::new(&mut commands, &asset_server, (*kind).clone(), Vec2::splat(120.0))
                    .pos(cursor.0);
                let gate = gate.spawn(&mut commands).id();

                moving_gate.0 = Some((gate, Vec2::ZERO));
            },
        }
    }
}

fn create_input_ui(mut commands: Commands, asset_server: Res<AssetServer>) {}
