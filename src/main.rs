mod constants;
mod cursor;
mod node;
mod edge;
mod gate;
mod ui;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use constants::Colors;
use cursor::CursorPlugin;

use gate::GatePlugin;
use node::{NodePlugin, NodeSpawner};
use edge::EdgePlugin;
use ui::UiBuilder;

fn startup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let _input3 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0,  80.0))).id();
    let _input1 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0,  40.0))).id();
    let _input2 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0, -40.0))).id();
    let _input4 = commands.spawn(NodeSpawner::from_pos(Vec2::new(-500.0, -80.0))).id();
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
