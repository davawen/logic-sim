#![allow(unused)]

mod cursor;
mod node;
mod edge;
mod gate;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Material2d, Mesh2dHandle}, input::mouse::MouseButtonInput, math::Vec3Swizzles, render::mesh::VertexAttributeValues};
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}};
use cursor::{Cursor, CursorPlugin};

use gate::{ GateBundle, GateType, GatePlugin };
use node::{Node, NodePlugin, NodeSpawner};
use edge::{Edge, EdgePlugin, EdgeBundle};

const RADIUS: f32 = 15.0;

struct NodeColors {
    off: Color,
    on: Color 
}

const NODE_COLORS: NodeColors = NodeColors {
    on: Color::rgb(0.9, 0.3, 0.3),
    off: Color::DARK_GRAY 
};

impl NodeColors {
    fn value(&self, v: bool) -> Color {
        if v { self.on } else { self.off }
    }

    fn highlighted(&self, v: bool) -> Color {
        self.value(v) + Color::WHITE*0.1
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let input1 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0, 50.0))).id();
    let input2 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0, -50.0))).id();

    let or_gate = GateBundle::new(&mut commands, &asset_server, GateType::Or, Vec2::splat(120.0)).pos(Vec2::new(-200.0, 100.0));

    // commands.spawn(EdgeBundle::new(input1, or_gate.gate.inputs[0]));
    // commands.spawn(EdgeBundle::new(input2, or_gate.gate.inputs[1]));

    let and_gate = GateBundle::new(&mut commands, &asset_server, GateType::And, Vec2::splat(120.0)).pos(Vec2::new(-200.0, -100.0));
    
    // commands.spawn(EdgeBundle::new(input1, and_gate.gate.inputs[0]));
    // commands.spawn(EdgeBundle::new(input2, and_gate.gate.inputs[1]));

    let not_gate = GateBundle::new(&mut commands, &asset_server, GateType::Not, Vec2::new(80.0, 40.0)).pos(Vec2::new(0.0, -100.0));

    // commands.spawn(EdgeBundle::new(and_gate.gate.output, not_gate.gate.inputs[0]));

    let and_gate2 = GateBundle::new(&mut commands, &asset_server, GateType::And, Vec2::splat(120.0)).pos(Vec2::new(120.0, 0.0));

    // commands.spawn(EdgeBundle::new(or_gate.gate.output, and_gate2.gate.inputs[0]));
    // commands.spawn(EdgeBundle::new(not_gate.gate.output, and_gate2.gate.inputs[1]));

    or_gate.spawn(&mut commands);
    and_gate.spawn(&mut commands);
    not_gate.spawn(&mut commands);
    and_gate2.spawn(&mut commands);

    // commands.spawn((
    //     Edge { from: a, to: b},
    // ));
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(CursorPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(EdgePlugin)
        .add_plugin(NodePlugin)
        .add_plugin(GatePlugin)
        .add_startup_system(startup)
        .run();
}
