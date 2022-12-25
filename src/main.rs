mod constants;
mod cursor;
mod node;
mod edge;
mod gate;
mod ui;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Material2d, Mesh2dHandle}, input::mouse::MouseButtonInput, math::Vec3Swizzles, render::mesh::VertexAttributeValues};
use bevy_prototype_lyon::{prelude::*, shapes::{Line, Circle, Rectangle}};
use constants::Colors;
use cursor::{Cursor, CursorPlugin};

use gate::{ GateBundle, GateType, GatePlugin };
use node::{Node, NodePlugin, NodeSpawner};
use edge::{Edge, EdgePlugin, EdgeBundle};
use ui::UiBuilder;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let input3 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0,  80.0))).id();
    let input1 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0,  40.0))).id();
    let input2 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0, -40.0))).id();
    let input4 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0, -80.0))).id();
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Colors::BG))
        .add_plugins(DefaultPlugins)
        .add_plugin(CursorPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(EdgePlugin)
        .add_plugin(NodePlugin)
        .add_plugin(GatePlugin)
        .add_plugin(UiBuilder)
        .add_startup_system(startup)
        .run();
}
