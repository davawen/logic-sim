use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}, entity::ShapeBundle};
use crate::{cursor::Cursor, NODE_COLORS};
use crate::{ RADIUS, NodeColors };

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(toggle_node_color)
            .add_system(toggle_node);
    }
}

#[derive(Bundle)]
pub struct NodeBundle {
    node: Node,
    shape: ShapeBundle
}

impl NodeBundle {
    pub fn new() -> Self {
        Self::from_pos(Vec2::ZERO)
    }

    pub fn from_pos(pos: Vec2) -> Self {
        Self {
            node: Node(false), 
            shape: GeometryBuilder::build_as(
                &Circle { center: Vec2::ZERO, radius: RADIUS },
                DrawMode::Fill(FillMode::color(Color::BLACK)), // will be set to NodeColors.off automatically
                Transform::from_translation(pos.extend(1.0))
            )
        }
    }
}

#[derive(Component)]
pub struct Node(pub bool);

fn toggle_node_color(mut commands: Commands, mut query: Query<(&Node, &mut DrawMode), Changed<Node>>) {
    for (node, mut draw_mode) in &mut query {
        if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {
            fill_mode.color = NODE_COLORS.value(node.0);
        }
    }
}

fn toggle_node(mut query: Query<(&mut Node, &Transform)>, cursor: Res<Cursor>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (mut node, transform) in &mut query {
            if cursor.0.distance_squared(transform.translation.truncate()) < RADIUS*RADIUS {
                node.0 = !node.0;
                break;
            }
        }
    }
}
