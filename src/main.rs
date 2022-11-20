#![allow(unused)]

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Material2d, Mesh2dHandle}, input::mouse::MouseButtonInput, math::Vec3Swizzles, render::mesh::VertexAttributeValues};

mod cursor;
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}};
use cursor::{Cursor, CursorPlugin};

#[derive(Component)]
struct Node(bool);

const RADIUS: f32 = 15.0;

#[derive(Component)]
struct Edge {
    from: Entity,
    to: Entity
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
    nodes: Query<(&Node, &Transform, ChangeTrackers<Node>, ChangeTrackers<Transform>)>,
    colors: Res<NodeColors>
) {
    for ( &Edge { from, to }, mut path, mut draw_mode ) in &mut edges {
        let (node, a, node_change, a_change) = nodes.get(from).unwrap();
        let (_, b, _, b_change) = nodes.get(to).unwrap();

        if node_change.is_changed() {
            if let DrawMode::Stroke(ref mut stroke_mode) = *draw_mode {
                stroke_mode.color = colors.value(node.0);
            }
        }

        if a_change.is_changed() || b_change.is_changed() {
            let line = Line( a.translation.truncate(), b.translation.truncate() );
            *path = ShapePath::build_as(&line);
        }

    }
}

fn toggle_node_color(mut commands: Commands, mut query: Query<(&Node, &mut DrawMode), Changed<Node>>, colors: Res<NodeColors>) {
    for (node, mut draw_mode) in &mut query {
        if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {
            fill_mode.color = colors.value(node.0);
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

fn move_node(mut query: Query<&mut Transform, With<Node>>, cursor: Res<Cursor>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.pressed(MouseButton::Middle) {
        for mut transform in &mut query {
            if cursor.0.distance_squared(transform.translation.truncate()) < RADIUS*RADIUS {
                *transform = transform.with_translation(cursor.0.extend(1.0));
                break;
            }
        }
    }
}

#[derive(Component)]
struct Gate {
    inputs: Vec<Entity>,
    output: Entity,
    kind: GateType
}

#[derive(Debug)]
enum GateType {
    And,
    Or,
    Xor,
    Not
}

fn process_gates(gates: Query<&Gate>, mut nodes: Query<&mut Node>) {
    for gate in gates.iter() {
        let get_node = |id: Entity| nodes.get(id).unwrap();

        use GateType::*;
        let output = match gate.kind {
            And => get_node(gate.inputs[0]).0 & get_node(gate.inputs[1]).0,
            Or => get_node(gate.inputs[0]).0 | get_node(gate.inputs[1]).0,
            Xor => get_node(gate.inputs[0]).0 ^ get_node(gate.inputs[1]).0,
            Not => !get_node(gate.inputs[0]).0
        };

        nodes.get_mut(gate.output).unwrap().0 = output;
    }
}

#[derive(Default, Resource)]
struct NodeColors {
    off: Color,
    on: Color 
}

impl NodeColors {
    fn value(&self, v: bool) -> Color {
        if v { self.on } else { self.off }
    }
}

fn startup(mut commands: Commands, mut colors: Res<NodeColors>) {
    commands.spawn(Camera2dBundle::default());

    let a = commands.spawn((
        Node(false), 
        GeometryBuilder::build_as(
            &Circle { center: Vec2::ZERO, radius: RADIUS },
            DrawMode::Fill(FillMode::color(colors.off)),
            Transform::from_translation(Vec3::new(-50.0, 50.0, 1.0))
        )
    )).id();

    let b = commands.spawn((
        Node(false), 
        GeometryBuilder::build_as(
            &Circle { center: Vec2::ZERO, radius: RADIUS },
            DrawMode::Fill(FillMode::color(colors.off)),
            Transform::from_translation(Vec3::new(-50.0, -50.0, 1.0))
        )
    )).id();

    let c = commands.spawn((
        Node(false), 
        GeometryBuilder::build_as(
            &Circle { center: Vec2::ZERO, radius: RADIUS },
            DrawMode::Fill(FillMode::color(colors.off)),
            Transform::from_translation(Vec3::new(50.0, 0.0, 1.0))
        )
    )).id();

    commands.spawn((
        Gate {
            kind: GateType::Or,
            inputs: vec![a, b],
            output: c
        }, 
        GeometryBuilder::build_as(
            &Rectangle { origin: RectangleOrigin::Center, extents: Vec2::new(120.0, 120.0) },
            DrawMode::Fill(FillMode::color(Color::PURPLE)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
        )
    ));

    // commands.spawn((
    //     Edge { from: a, to: b},
    //     GeometryBuilder::build_as(
    //         &Line(Vec2::ZERO, Vec2::ZERO),
    //         DrawMode::Stroke(StrokeMode::new(colors.off, 5.0)),
    //         Transform::default()
    //     )
    // ));
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(NodeColors{ on: Color::rgb(0.9, 0.3, 0.3), off: Color::DARK_GRAY })
        .add_plugins(DefaultPlugins)
        .add_plugin(CursorPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup)
        .add_system(toggle_node)
        .add_system(toggle_node_color)
        .add_system(move_node)
        .add_system(propagate)
        .add_system(update_edge_lines)
        .add_system(process_gates)
        .run();
}
