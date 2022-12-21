use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}, entity::ShapeBundle};

use crate::{node::{Node, HoveredNode}, NODE_COLORS, cursor::Cursor, RADIUS};
use crate::NodeColors;

pub struct EdgePlugin;

impl Plugin for EdgePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedNode(None))
            .add_system(propagate)
            .add_system(update_edge_lines)
            .add_system(create_edges);
    }
}

#[derive(Bundle)]
pub struct EdgeBundle {
    pub edge: Edge,
    shape: ShapeBundle
}

impl EdgeBundle {
    pub fn new(a: Entity, b: Entity) -> Self {
        Self {
            edge: Edge { from: a, to: b },
            shape: GeometryBuilder::build_as(
                &Line(Vec2::ZERO, Vec2::ZERO),
                DrawMode::Stroke(StrokeMode::new(NODE_COLORS.off, 5.0)),
                Transform::default()
            )
        }
    }
}

#[derive(Component)]
pub struct Edge {
    pub from: Entity,
    pub to: Entity
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

fn update_edge_lines(
    mut edges: Query<(&Edge, &mut Path, &mut DrawMode, ChangeTrackers<Edge>)>,
    nodes: Query<(&Node, &Transform, ChangeTrackers<Node>, ChangeTrackers<Transform>)>
) {
    for ( &Edge { from, to }, mut path, mut draw_mode, edge_changed ) in &mut edges {
        let (node, a, node_change, a_change) = nodes.get(from).unwrap();
        let (_, b, _, b_change) = nodes.get(to).unwrap();

        if node_change.is_changed() {
            if let DrawMode::Stroke(ref mut stroke_mode) = *draw_mode {
                stroke_mode.color = NODE_COLORS.value(node.0);
            }
        }

        if a_change.is_changed() || b_change.is_changed() || edge_changed.is_changed() {
            let line = Line( a.translation.truncate(), b.translation.truncate() );
            *path = ShapePath::build_as(&line);
        }

    }
}

/// Holds a reference to the edge the mouse is currently hovering over
// #[derive(Resource)]
// struct HoveredEdge(Option<Entity>);
//
// fn hover_edge(edges: Query<&Edge>, nodes: Query<&Transform, With<Node>>, mut hovered: ResMut<HoveredEdge>, cursor: Res<Cursor>) {
//     for &Edge { from, to } in edges.iter() {
//         let Ok([ from, to ]) = nodes.get_many([from, to]) else { continue };
//
//         // get distance from mouse to line
//         let distance = {
//
//             let dist_from_to = from.translation.distance(to.translation);
//         };
//     }
// }

/// This holds a reference to the first node selected when creating an edge between two nodes
#[derive(Resource)]
struct SelectedNode(Option<Entity>);

fn create_edges(mut commands: Commands, mut selected_node: ResMut<SelectedNode>, hovered: Res<HoveredNode>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Right) {
        selected_node.0 = hovered.0;
    }
    else if mouse_input.just_released(MouseButton::Right) {
        let Some(selected) = selected_node.0 else { return };

        if let Some(hovered) = hovered.0 { 
            commands.spawn(EdgeBundle::new(selected, hovered));
        };
        selected_node.0 = None;
    }
}

fn delete_edges(mut commands: Commands, edges: Query<(&Transform, Entity), With<Edge>>, cursor: Res<Cursor>, mouse_input: Res<Input<MouseButton>>) {

}
