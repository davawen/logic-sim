use std::time::Duration;

use bevy::{prelude::*, ecs::query::QueryEntityError};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::*,
    shapes::Line,
};

use crate::{
    cursor::Cursor,
    node::{HoveredNode, Node},
    constants::{Colors, Depth}
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
            .add_system(delete_edges)
            .add_system(cleanup_edges);
    }
}

#[derive(Bundle)]
pub struct EdgeBundle {
    pub edge: Edge,
    shape: ShapeBundle,
    timer: EdgeTimer
}

impl EdgeBundle {
    pub fn new(a: Entity, b: Entity) -> Self {
        let mut timer = EdgeTimer(Timer::from_seconds(0.1, TimerMode::Once));
        timer.0.set_elapsed(Duration::from_millis(100));

        Self {
            edge: Edge { from: a, to: b },
            shape: GeometryBuilder::build_as(
                &Line(Vec2::ZERO, Vec2::ZERO),
                DrawMode::Stroke(StrokeMode::new(Colors::OFF, 5.0)),
                Transform::from_xyz(0.0, 0.0, Depth::EDGE),
            ),
            timer
        }
    }
}

#[derive(Component)]
pub struct Edge {
    pub from: Entity,
    pub to: Entity,
}

/// Determines how much a signal progressed through an edge
#[derive(Component)]
pub struct EdgeTimer(pub Timer);

fn propagate(mut query: Query<(&Edge, &mut EdgeTimer)>, mut nodes: Query<&mut Node>, time: Res<Time>) {
    for ( &Edge { from, to }, mut timer ) in &mut query {
        let Ok([a, mut b]) = nodes.get_many_mut([ from, to ]) else { continue };

        if timer.0.finished() && b.0 != a.0 {
            timer.0.reset();
        }
        else if !timer.0.finished() {
            timer.0.tick(time.delta());
        }

        if timer.0.just_finished() {
            b.0 = a.0;
        }
    }
}

// Holds a reference to the edge the mouse is currently hovering over
#[derive(Resource)]
struct HoveredEdge(Option<Entity>);

fn hover_edge(
    edges: Query<(Entity, &Edge)>,
    nodes: Query<&GlobalTransform, With<Node>>,
    mut hovered: ResMut<HoveredEdge>,
    hovered_node: Res<HoveredNode>,
    cursor: Res<Cursor>,
) {
    // Disallow selecting an edge when hovering over a node
    if hovered_node.0.is_none() {
        for (edge, &Edge { from, to }) in edges.iter() {
            let Ok([ a, b ]) = nodes.get_many([from, to]) else { continue };

            let a = a.translation();
            let b = b.translation();
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
    nodes: Query<(&GlobalTransform, ChangeTrackers<GlobalTransform>), With<Node>>,
) {
    for (&Edge { from, to }, mut path, edge_change) in &mut edges {
        let Ok([( a, a_change ), (b, b_change)]) = nodes.get_many([from, to]) else { return };

        if a_change.is_changed() || b_change.is_changed() || edge_change.is_changed() {
            let line = Line(a.translation().truncate(), b.translation().truncate());
            *path = ShapePath::build_as(&line);
        }
    }
}

fn set_edge_color(
    mut edges: Query<(Entity, &Edge, &EdgeTimer, &mut DrawMode)>,
    nodes: Query<&Node>,
    hovered: Res<HoveredEdge>,
) {
    for (edge, &Edge { from, to }, timer, mut draw_mode) in &mut edges {
        let Ok([ from, to]) = nodes.get_many([from, to]) else { return };

        let DrawMode::Stroke(ref mut stroke_mode) = *draw_mode else { return };

        let func = if Some(edge) == hovered.0 {
            Colors::highlighted
        } else {
            Colors::value
        };

        // Linear interpolation based on timer
        stroke_mode.color = func(!from.0) * timer.0.percent_left() + func(from.0) * timer.0.percent();
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
    mouse_input: Res<Input<MouseButton>>,
    hovered_edge: Res<HoveredEdge>,
) {
    if mouse_input.just_released(MouseButton::Right) {
        let Some(hovered) = hovered_edge.0 else { return };

        commands.entity(hovered).despawn();
    }
}

/// Remove edges whose nodes have been deleted
fn cleanup_edges(
    mut commands: Commands,
    edges: Query<(Entity, &Edge)>,
    nodes: Query<(), With<Node>>
) {
    for (entity, &Edge { from, to }) in edges.iter() {
        if let Err(QueryEntityError::NoSuchEntity(_)) = nodes.get_many([from, to]) {
            commands.get_entity(entity).unwrap().despawn();
        }
    }
}
