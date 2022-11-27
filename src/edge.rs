use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}, entity::ShapeBundle};

use crate::{node::Node, NODE_COLORS};
use crate::NodeColors;

pub struct EdgePlugin;

impl Plugin for EdgePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(propagate)
            .add_system(update_edge_lines);
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
        let mut a = nodes.get(from).unwrap().0;
        let mut b = nodes.get_mut(to).unwrap();

        if b.0 != a {
            b.0 = a;
        }
    }
}

fn update_edge_lines(
    mut edges: Query<(&Edge, &mut Path, &mut DrawMode)>,
    nodes: Query<(&Node, &Transform, ChangeTrackers<Node>, ChangeTrackers<Transform>)>
) {
    for ( &Edge { from, to }, mut path, mut draw_mode ) in &mut edges {
        let (node, a, node_change, a_change) = nodes.get(from).unwrap();
        let (_, b, _, b_change) = nodes.get(to).unwrap();

        if node_change.is_changed() {
            if let DrawMode::Stroke(ref mut stroke_mode) = *draw_mode {
                stroke_mode.color = NODE_COLORS.value(node.0);
            }
        }

        if a_change.is_changed() || b_change.is_changed() {
            let line = Line( a.translation.truncate(), b.translation.truncate() );
            *path = ShapePath::build_as(&line);
        }

    }
}

