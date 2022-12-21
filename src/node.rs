use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}, entity::ShapeBundle};
use crate::{cursor::Cursor, NODE_COLORS};
use crate::{ RADIUS, NodeColors };

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HoveredNode( None ))
            .add_system(hover_node)
            .add_system(set_node_color)
            .add_system(toggle_node);
    }
}

#[derive(Bundle)]
pub struct NodeSpawner {
    node: Node,
    shape: ShapeBundle
}

impl NodeSpawner {
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

#[derive(Component, Clone)]
pub struct Node(pub bool);

/// This holds a reference to the node that is currently hovered over by the mouse
#[derive(Resource)]
pub struct HoveredNode(pub Option<Entity>);

fn hover_node(query: Query<(Entity, &Transform), With<Node>>, mut hovered: ResMut<HoveredNode>, cursor: Res<Cursor>) {
    for (entity, transform) in query.iter() {
        if cursor.0.distance_squared(transform.translation.truncate()) < RADIUS*RADIUS {
            hovered.0 = Some(entity);
            return;
        }
    }
    hovered.0 = None;
}

fn set_node_color(mut commands: Commands, mut query: Query<(Entity, &Node, &mut DrawMode)>, hovered: Res<HoveredNode>) {
    for (entity, node, mut draw_mode) in &mut query {
        let DrawMode::Fill(ref mut fill_mode) = *draw_mode else { return };
        
        if Some(entity) == hovered.0 {
            fill_mode.color = NODE_COLORS.highlighted(node.0);
        }
        else {
            fill_mode.color = NODE_COLORS.value(node.0);
        }
    }
}

fn toggle_node(mut query: Query<&mut Node>, hovered: Res<HoveredNode>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Some(hovered) = hovered.0 else { return };
        let Ok(mut node) = query.get_mut(hovered) else { return };
        node.0 = !node.0;
    }
}
