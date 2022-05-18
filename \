#![allow(dead_code, unused)]

use rand::{thread_rng, Rng};
use std::{array, mem::size_of, error::Error, collections::hash_map::*};

use sfml::{graphics::*, system::*, window::*};
use ghost_cell::{GhostToken, GhostCell, self};

#[derive(Default, Clone, Copy)]
struct Id<T> {
    id: usize,
    _marker: std::marker::PhantomData<T>
}

impl<T> Id<T> {
    fn new() -> Self {
        Id {
            id: thread_rng().gen_range(0..usize::MAX),
            _marker: std::marker::PhantomData
        }
    }

    fn get<'a>(&self, map: &'a HashMap<usize, T>) -> Option<&'a T> {
        map.get(&self.id)
    }

    fn get_mut<'a>(&self, map: &'a mut HashMap<usize, T>) -> Option<&'a mut T> {
        map.get_mut(&self.id)
    }
}

trait HashId<T> {
    fn get(&self, id: Id<T>) -> Option<&T>;
    fn get_mut(&mut self, id: Id<T>) -> Option<&mut T>;
    fn insert(&mut self, value: T) -> Id<T>;
}

impl<T> HashId<T> for HashMap<usize, T> {
    fn get(&self, id: Id<T>) -> Option<&T> {
       self.get(&id.id) 
    }

    fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.get_mut(&id.id)
    }

    fn insert(&mut self, value: T) -> Id<T> {
        let new_id = Id::new();

        self.insert(new_id.id, value);

        new_id
    }
}


#[derive(Default)]
struct Pin {
    value: bool,
    edge: Option<Id<Edge>>,
    gate: Option<Id<Gate>>
}

impl Pin {
    fn new() -> Self {
        Pin { ..Default::default() }
    }

    fn create(pins: &'x mut Vec<Pin<'x>>) -> PinRef<'x> {
        GhostCell::from_mut(
            pins.push_last( Pin::new() )
        )
    }
}

struct Edge {
    from: Id<Pin>,
    to: Id<Pin>
}

struct Gate<'x> {
    inputs: [ PinRef<'x>; 2 ],
    output: PinRef<'x>,
    op: Box<dyn Fn(bool, bool) -> bool>
}

impl<'x> Gate<'x> {
    // fn new(graph: &mut GhostToken<'x>, pins: &'x mut Vec<Pin<'x>>) -> Self {
    //     let input1 = Pin::create(pins);
    //     let input2 = Pin::create(pins);
    //     let output = Pin::create(pins);

    //     Gate {
    //         inputs: [ input1, input2 ],
    //         output: output,
    //         op: Box::new(|a, b|{ a && b })
    //     }
    // }

    fn compute(&mut self, graph: &mut GhostToken<'x>) {
        let result = (*self.op)(self.inputs[0].borrow(graph).value, self.inputs[1].borrow(graph).value);
    }
}

fn main() {


    let mut window = RenderWindow::new(
        (800, 600),
        "Mouse events",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_mouse_cursor_visible(false);
    window.set_vertical_sync_enabled(true);

    let font = Font::from_file("/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf").unwrap();
    let mut circle = CircleShape::new(4., 30);
    let mut texts: Vec<Text> = Vec::new();
    let mut mp_text = Text::new("", &font, 14);
    let mut cursor_visible = false;
    let mut grabbed = false;
    macro_rules! push_text {
        ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*) => {
            let mut text = Text::new(&format!($fmt, $($arg)*), &font, 14);
            text.set_position(($x as f32, $y as f32));
            texts.push(text);
        }
    }

    loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => return,
                Event::MouseWheelScrolled { wheel, delta, x, y } => {
                    push_text!(x, y, "Scroll: {:?}, {}, {}, {}", wheel, delta, x, y);
                }
                Event::MouseButtonPressed { button, x, y } => {
                    push_text!(x, y, "Press: {:?}, {}, {}", button, x, y);
                }
                Event::MouseButtonReleased { button, x, y } => {
                    push_text!(x, y, "Release: {:?}, {}, {}", button, x, y);
                }
                Event::KeyPressed { code, .. } => {
                    if code == Key::W {
                        window.set_mouse_position(Vector2i::new(400, 300));
                    } else if code == Key::D {
                        let dm = VideoMode::desktop_mode();
                        let center = Vector2i::new(dm.width as i32 / 2, dm.height as i32 / 2);
                        mouse::set_desktop_position(center);
                    } else if code == Key::V {
                        cursor_visible = !cursor_visible;
                        window.set_mouse_cursor_visible(cursor_visible);
                    } else if code == Key::G {
                        grabbed = !grabbed;
                        window.set_mouse_cursor_grabbed(grabbed);
                    }
                }
                _ => {}
            }
        }

        let mp = window.mouse_position();
        let dmp = mouse::desktop_position();
        let cur_vis_msg = if cursor_visible {
            "visible"
        } else {
            "invisible"
        };
        let grab_msg = if grabbed { "grabbed" } else { "not grabbed" };
        mp_text.set_string(&format!(
            "x: {}, y: {} (Window)\n\
             x: {}, y: {} (Desktop)\n\
             [{}] [{}] ('V'/'G') to toggle\n\
             'W' to center mouse on window\n\
             'D' to center mouse on desktop",
            mp.x, mp.y, dmp.x, dmp.y, cur_vis_msg, grab_msg
        ));

        circle.set_position((mp.x as f32, mp.y as f32));

        window.clear(Color::BLACK);
        // Push texts out of each other's way
        for i in (0..texts.len()).rev() {
            for j in (0..i).rev() {
                if let Some(intersect) = texts[i]
                    .global_bounds()
                    .intersection(&texts[j].global_bounds())
                {
                    texts[j].move_((0., -intersect.height));
                }
            }
        }
        texts.retain(|txt| txt.fill_color().alpha() > 0);
        for txt in &mut texts {
            let mut color = txt.fill_color();
            *color.alpha_mut() -= 1;
            txt.set_fill_color(color);
            window.draw(txt);
        }
        if !cursor_visible {
            window.draw(&circle);
        }
        window.draw(&mp_text);
        window.display();
    }
}
