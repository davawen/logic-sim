use bevy::prelude::*;

pub struct Colors;

impl Colors {
    pub const ON: Color = Color::rgb(0.9, 0.3, 0.3);
    pub const OFF: Color = Color::DARK_GRAY;

    pub const BG: Color = Color::rgb(0.4, 0.4, 0.4);
    pub const UI_BG: Color = Color::rgb(0.3, 0.3, 0.3);
    
    pub fn value(v: bool) -> Color {
        if v { Self::ON } else { Self::OFF }
    }

    pub fn highlighted(v: bool) -> Color {
        Self::value(v) + Color::WHITE*0.1
    }
}

pub const RADIUS: f32 = 15.0;

pub struct Depth;

impl Depth {
    pub const GATE: f32 = 0.0; // Farthest back
    pub const EDGE: f32 = 1.0; // In front of gate
    pub const NODE: f32 = 2.0; // In front of edges
    pub const TEXT: f32 = 3.0; // In front of every "background" element
    pub const UI: f32 = 10.0; // In front of everything
}
