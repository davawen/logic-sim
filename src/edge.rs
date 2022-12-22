use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::*,
    shapes::{Circle, Line, Rectangle},
};

use crate::{
    cursor::Cursor,
    node::{HoveredNode, Node},
    constants::{Colors, RADIUS, Depth}
};

pub struct EdgePlugin;

impl Plugin for EdgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedNode(None))
            .insert_non_send_resource(HoveredEdge(None))
            .add_system(propagate)
            .add_system(move_edge)
            .add_system(set_edge_color)
            .add_system(hover_edge)
            .add_system(create_edges)
            .add_system(delete_edges);
    }
}

#[derive(Bundle)]
pub struct EdgeBundle {
    pub edge: Edge,
    shape: ShapeBundle,
}

impl EdgeBundle {
    pub fn new(a: Entity, b: Entity) -> Self {
        Self {
            edge: Edge { from: a, to: b },
            shape: GeometryBuilder::build_as(
                &Line(Vec2::ZERO, Vec2::ZERO),
                DrawMode::Stroke(StrokeMode::new(Colors::OFF, 5.0)),
                Transform::from_xyz(0.0, 0.0, Depth::EDGE),
            ),
        }
    }
}

#[derive(Component)]
pub struct Edge {
    pub from: Entity,
    pub to: Entity,
}

fn propagate(query: Query<&Edge>, mut nodes: Query<&mut Node>) {
    for &Edge { from, to } in query.iter() {
        let a = nodes.get(from).unwrap().clone();
        let mut b = nodes.get_mut(to).unwrap();

        if b.0 != a.0 {
            b.0 = a.0;
        }
    }
}

// Holds a reference to the edge the mouse is currently hovering over
#[derive(Resource)]
struct HoveredEdge(Option<Entity>);

fn hover_edge(
    edges: Query<(Entity, &Edge)>,
    nodes: Query<&Transform, With<Node>>,
    mut hovered: ResMut<HoveredEdge>,
    hovered_node: Res<HoveredNode>,
    cursor: Res<Cursor>,
) {
    // Disallow selecting an edge when hovering over a node
    if hovered_node.0.is_none() {
        for (edge, &Edge { from, to }) in edges.iter() {
            let Ok([ a, b ]) = nodes.get_many([from, to]) else { continue };

            let a = a.translation;
            let b = b.translation;
            let p = cursor.0;

            // get distance from mouse to line
            let distance = {
                let dist_p_a = p.distance(a.truncate());
                let dist_p_b = p.distance(b.truncate());

                let dist_a_b = a.distance(b);
                let dist_to_line =
                    ((b.x - a.x) * (a.y - p.y) - (a.x - p.x) * (b.y - a.y)).abs() / dist_a_b;

                dist_to_line.min(dist_p_a).min(dist_p_b)
            };

            if distance < 5.0 {
                // width of edge
                hovered.0 = Some(edge);
                return;
            }
        }
    };
    hovered.0 = None;
}

fn move_edge(
    mut edges: Query<(&Edge, &mut Path, ChangeTrackers<Edge>)>,
    nodes: Query<(&Transform, ChangeTrackers<Transform>), With<Node>>,
) {
    for (&Edge { from, to }, mut path, edge_change) in &mut edges {
        let Ok([( a, a_change ), (b, b_change)]) = nodes.get_many([from, to]) else { return };

        if a_change.is_changed() || b_change.is_changed() || edge_change.is_changed() {
            let line = Line(a.translation.truncate(), b.translation.truncate());
            *path = ShapePath::build_as(&line);
        }
    }
}

fn set_edge_color(
    mut edges: Query<(Entity, &Edge, &mut DrawMode)>,
    nodes: Query<&Node>,
    hovered: Res<HoveredEdge>,
) {
    for (edge, &Edge { from, to }, mut draw_mode) in &mut edges {
        let Ok([ from, to]) = nodes.get_many([from, to]) else { return };

        let DrawMode::Stroke(ref mut stroke_mode) = *draw_mode else { return };

        if Some(edge) == hovered.0 {
            stroke_mode.color = Colors::highlighted(from.0);
        } else {
            stroke_mode.color = Colors::value(from.0);
        }
    }
}

/// This holds a reference to the first node selected when creating an edge between two nodes
#[derive(Resource)]
struct SelectedNode(Option<Entity>);

fn create_edges(
    mut commands: Commands,
    mut selected_node: ResMut<SelectedNode>,
    hovered: Res<HoveredNode>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        selected_node.0 = hovered.0;
    } else if mouse_input.just_released(MouseButton::Right) {
        let Some(selected) = selected_node.0 else { return };

        if let Some(hovered) = hovered.0 {
            commands.spawn(EdgeBundle::new(selected, hovered));
        };
        selected_node.0 = None;
    }
}

fn delete_edges(
    mut commands: Commands,
    edges: Query<(&Transform, Entity), With<Edge>>,
    mouse_input: Res<Input<MouseButton>>,
    hovered_edge: Res<HoveredEdge>,
) {
    if mouse_input.just_released(MouseButton::Right) {
        let Some(hovered) = hovered_edge.0 else { return };

        commands.entity(hovered).despawn();
    }
}
