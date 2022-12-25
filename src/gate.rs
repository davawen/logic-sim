use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Rectangle;

use crate::constants::Depth;
use crate::cursor::Cursor;
use crate::node::{Node, NodeSpawner};

pub struct GatePlugin;

impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovingGate(None))
            .add_system(move_gate)
            // .add_system(move_gate_nodes)
            .add_system(process_gates);
    }
}

// #[derive(Bundle)]
pub struct GateBundle {
    pub size: Vec2,
    pub kind: GateType,
    shape: ShapeBundle,
    text: Text2dBundle,
}

impl GateBundle {
    pub fn new(asset_server: &Res<AssetServer>, kind: GateType, size: Vec2) -> Self {
        let kind_name = kind.as_str();

        Self {
            size,
            kind,
            shape: GeometryBuilder::build_as(
                &Rectangle {
                    origin: RectangleOrigin::Center,
                    extents: size,
                },
                DrawMode::Fill(FillMode::color(Color::PURPLE)),
                Transform::from_xyz(0.0, 0.0, Depth::GATE),
            ),
            text: Text2dBundle {
                text: Text::from_section(
                    kind_name,
                    TextStyle {
                        font: asset_server.load("FiraCode.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                }),
                transform: Transform::from_xyz(0.0, 0.0, Depth::TEXT),
                ..Default::default()
            },
        }
    }

    pub fn pos(mut self, pos: Vec2) -> Self {
        self.shape.transform.translation = pos.extend(0.0);

        self
    }

    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
    ) -> bevy::ecs::system::EntityCommands<'w, 's, 'a> {
        let num_inputs = self.kind.num_inputs();
        let inputs = (0..self.kind.num_inputs())
            .rev()
            .map(|idx| {
                let node = NodeSpawner::from_pos(Vec2::new(
                    -self.size.x / 2.0,
                    (idx as f32 + 1.0) / (num_inputs as f32 + 1.0) * self.size.y
                        - self.size.y / 2.0,
                ));
                commands.spawn(node).id()
            })
            .collect::<Vec<_>>();

        let output = commands
            .spawn(NodeSpawner::from_pos(Vec2::new(self.size.x / 2.0, 0.0)))
            .id();

        let mut bund = commands.spawn((
            Gate {
                inputs: inputs.clone(),
                output,
                kind: self.kind,
                size: self.size,
            },
            self.shape,
        ));

        bund.push_children(&inputs)
            .add_child(output)
            .with_children(|b| {
                b.spawn(self.text);
            });

        bund
    }
}

#[derive(Component)]
pub struct Gate {
    pub inputs: Vec<Entity>,
    pub output: Entity,
    pub size: Vec2,
    pub kind: GateType,
}

#[derive(Debug, Clone)]
pub enum GateType {
    And,
    Or,
    Xor,
    Not,
}

impl GateType {
    pub fn as_str(&self) -> &'static str {
        use GateType::*;
        match self {
            And => "And",
            Or => "Or",
            Xor => "Xor",
            Not => "Not",
        }
    }

    pub fn num_inputs(&self) -> usize {
        use GateType::*;
        match self {
            And | Or | Xor => 2,
            Not => 1,
        }
    }
}

/// Holds a reference to the currently moving gate, as well as the offset it was selected at
#[derive(Resource)]
pub struct MovingGate(pub Option<(Entity, Vec2)>);

fn snap_vec(v: Vec2, grid_size: Vec2) -> Vec2 {
    (v / grid_size).round() * grid_size
}

fn move_gate(
    mut query: Query<(Entity, &mut Transform, &Gate)>,
    mut selected: ResMut<MovingGate>,
    cursor: Res<Cursor>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (entity, transform, gate) in query.iter() {
            let p = cursor.0;
            let pos = transform.translation.truncate();
            let size = gate.size;

            if p.cmpgt(pos - size / 2.0).all() && p.cmplt(pos + size / 2.0).all() {
                selected.0 = Some((entity, pos - p));
                break;
            }
        }
    } else if mouse_input.just_released(MouseButton::Left) {
        selected.0 = None;
    }

    if let Some((entity, offset)) = selected.0 {
        let Ok(( _, mut transform, _ )) = query.get_mut(entity) else { return };
        *transform = transform.with_translation(
            snap_vec(cursor.0 + offset, Vec2::new(20.0, 20.0)).extend(Depth::GATE),
        );
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
            Not => !get_node(gate.inputs[0]).0,
        };

        nodes.get_mut(gate.output).unwrap().0 = output;
    }
}
