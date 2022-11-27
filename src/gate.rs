use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Rectangle;

use crate::cursor::Cursor;
use crate::node::{Node, NodeBundle};

pub struct GatePlugin;

impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_gate)
            .add_system(move_gate_nodes)
            .add_system(process_gates);
    }
}

#[derive(Bundle)]
pub struct GateBundle {
    pub gate: Gate,
    shape: ShapeBundle
}

impl GateBundle {
    pub fn new(commands: &mut Commands, kind: GateType, size: Vec2) -> Self {
        use GateType::*;
        let inputs = match kind {
            And | Or | Xor => vec![ NodeBundle::new(), NodeBundle::new() ],
            Not => vec![ NodeBundle::new() ]
        };

        let inputs = inputs.into_iter().map(|bundle| commands.spawn(bundle).id() ).collect::<Vec<_>>();
        let output = commands.spawn(NodeBundle::new()).id();

        Self {
            gate: Gate {
                inputs,
                output,
                size,
                kind
            }, 
            shape: GeometryBuilder::build_as(
                &Rectangle { origin: RectangleOrigin::Center, extents: size },
                DrawMode::Fill(FillMode::color(Color::PURPLE)),
                Transform::IDENTITY
            )
        }
    }

    pub fn pos(mut self, pos: Vec2) -> Self {
        self.shape.transform.translation = pos.extend(0.0);

        self
    }
}

#[derive(Component)]
pub struct Gate {
    pub inputs: Vec<Entity>,
    pub output: Entity,
    pub size: Vec2,
    pub kind: GateType
}

#[derive(Debug)]
pub enum GateType {
    And,
    Or,
    Xor,
    Not
}

fn move_gate(mut query: Query<(&mut Transform, &Gate)>, cursor: Res<Cursor>, mouse_input: Res<Input<MouseButton>>) {
    if mouse_input.pressed(MouseButton::Middle) {
        for (mut transform, gate) in &mut query {
            let p = cursor.0;
            let pos = transform.translation.truncate();
            let size = gate.size;

            if p.cmpgt(pos - size/2.0).all() && p.cmplt(pos + size/2.0).all() {
                *transform = transform.with_translation(cursor.0.extend(0.0));
                break;
            }
        }
    }
}

fn move_gate_nodes(query: Query<(&Transform, &Gate), Changed<Transform>>, mut nodes: Query<&mut Transform, (With<Node>, Without<Gate>)>) {
    for (transform, gate) in query.iter() {
        let mut idx = gate.inputs.len();
        let mut iter = nodes.iter_many_mut(&gate.inputs);
        while let Some(mut input_transform) = iter.fetch_next() {
            idx -= 1;
            input_transform.translation = transform.translation + Vec3::new( -gate.size.x/2.0, (idx as f32 + 1.0)/(gate.inputs.len() as f32 + 1.0) * gate.size.y - gate.size.y/2.0, 1.0);
        }

        let mut output_transform = nodes.get_mut(gate.output).unwrap();
        output_transform.translation = transform.translation + Vec3::new( gate.size.x/2.0, 0.0, 1.0);
    }
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
