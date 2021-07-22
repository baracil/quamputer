use raylib::camera::Camera2D;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::RaylibHandle;

#[derive(Default, Clone)]
pub struct Displacement {
    pub starting_pos: Vector2,
    pub current_pos: Vector2,
    pub delta: Vector2,
}


impl Displacement {
    pub fn init(&mut self, position: Vector2) {
        self.starting_pos = position;
        self.current_pos = position;
        self.delta.x = 0.0;
        self.delta.y = 0.0;
    }
    pub fn set_current_position(&mut self, position: Vector2) {
        self.current_pos = position;
        self.delta = self.starting_pos - self.current_pos;
    }
}
