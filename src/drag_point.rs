use sdl2::event::Event;
use sdl2::mouse::MouseState;

pub struct DragPoint {
    pub pos: (i32, i32),
    pub clicked_pos: (i32, i32),
    pub held_down: bool,
    pub radius: f32,
}

impl DragPoint {
    pub fn new(pos: (i32, i32), radius: f32) -> DragPoint {
        DragPoint {
            pos,
            clicked_pos: (0, 0),
            held_down: false,
            radius,
        }
    }

    pub fn event_update(&mut self, event: &Event, mouse_state: &MouseState) {
        match event {
            Event::MouseButtonDown { .. } => {
                self.held_down = true;
                self.clicked_pos = (mouse_state.x(), mouse_state.y())
            }
            Event::MouseButtonUp { .. } => self.held_down = false,
            _ => {}
        }
    }

    pub fn frame_update(&mut self, mouse_state: &MouseState) {
        if self.held_down {
            let mouse_pos = (mouse_state.x(), mouse_state.y());
            self.pos = (
                self.pos.0 + mouse_pos.0 - self.clicked_pos.0,
                self.pos.1 + mouse_pos.1 - self.clicked_pos.1,
            );
            self.clicked_pos = mouse_pos
        }
    }
}
