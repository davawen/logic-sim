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
            .add_system(move_gate_nodes)
            .add_system(process_gates);
    }
}

// #[derive(Bundle)]
pub struct GateBundle {
    pub gate: Gate,
    shape: ShapeBundle,
    text: Text2dBundle,
}

impl GateBundle {
    pub fn new(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        kind: GateType,
        size: Vec2,
    ) -> Self {
        use GateType::*;
        let inputs = match kind {
            And | Or | Xor => vec![NodeSpawner::new(), NodeSpawner::new()],
            Not => vec![NodeSpawner::new()],
        };

        let inputs = inputs
            .into_iter()
            .map(|bundle| commands.spawn(bundle).id())
            .collect::<Vec<_>>();
        let output = commands.spawn(NodeSpawner::new()).id();

        let kind_name = kind.as_str();

        Self {
            gate: Gate {
                inputs,
                output,
                size,
                kind,
            },
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
        let mut bund = commands.spawn((self.gate, self.shape));

        bund.with_children(|mut b| {
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
}

/// Holds a reference to the currently moving gate, as well as the offset it was selected at
#[derive(Resource)]
pub struct MovingGate(pub Option<(Entity, Vec2)>);

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
        *transform = transform.with_translation((cursor.0 + offset).extend(Depth::GATE));
    }
}

fn move_gate_nodes(
    query: Query<(&Transform, &Gate), Changed<Transform>>,
    mut nodes: Query<&mut Transform, (With<Node>, Without<Gate>)>,
) {
    for (transform, gate) in query.iter() {
        let mut idx = gate.inputs.len();
        let mut iter = nodes.iter_many_mut(&gate.inputs);
        while let Some(mut input_transform) = iter.fetch_next() {
            idx -= 1;
            let offset = Vec2::new(
                -gate.size.x / 2.0,
                (idx as f32 + 1.0) / (gate.inputs.len() as f32 + 1.0) * gate.size.y
                    - gate.size.y / 2.0,
            );
            input_transform.translation =
                (transform.translation.truncate() + offset).extend(Depth::NODE)
        }

        let mut output_transform = nodes.get_mut(gate.output).unwrap();
        output_transform.translation = (transform.translation.truncate()
            + Vec2::new(gate.size.x / 2.0, 0.0))
        .extend(Depth::NODE);
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
