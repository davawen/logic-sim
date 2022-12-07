use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}, entity::ShapeBundle};

use crate::{node::Node, NODE_COLORS, cursor::Cursor, RADIUS};
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

#[derive(Resource)]
/// This holds a reference to the first node selected when creating an edge between two nodes
struct SelectedNode(Option<Entity>);

fn create_edges(mut commands: Commands, nodes: Query<(&Transform, Entity), With<Node>>, mut selected_node: ResMut<SelectedNode>, cursor: Res<Cursor>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.just_pressed(MouseButton::Right) {
        for (transform, entity) in nodes.iter() {
            if cursor.0.distance_squared(transform.translation.truncate()) < RADIUS*RADIUS {
                selected_node.0 = Some(entity);
                break;
            }
        }
    }
    else if mouse_input.just_released(MouseButton::Right) {
        if let Some(selected) = selected_node.0 {
            for (transform, entity) in nodes.iter() {
                if cursor.0.distance_squared(transform.translation.truncate()) < RADIUS*RADIUS {
                    commands.spawn(EdgeBundle::new(selected, entity));
                    break;
                }
            }
        }
        selected_node.0 = None;
    }
}

fn delete_edges(mut commands: Commands, edges: Query<(&Transform, Entity)>, cursor: Res<Cursor>, mouse_input: Res<Input<MouseButton>>) {

}
