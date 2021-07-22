use raylib::camera::Camera2D;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::RaylibHandle;

use crate::gui::displacement::Displacement;

#[derive(Clone)]
pub struct DragInfo {
    pub button_type: raylib::consts::MouseButton,
    pub use_space: bool,
    pub in_progress: bool,
    pub started: bool,
    pub done: bool,
    pub screen_displacement: Displacement,
    pub world_displacement: Displacement,
}

impl DragInfo {
    pub fn new(button_type: raylib::consts::MouseButton, use_space: bool) -> Self {
        return DragInfo {
            button_type,
            screen_displacement: Default::default(),
            world_displacement: Default::default(),
            in_progress: false,
            started: false,
            done: false,
            use_space,
        };
    }

    pub fn get_screen_displacement(&self) -> &Displacement {
        &self.screen_displacement
    }

    pub fn get_world_displacement(&self) -> &Displacement {
        &self.world_displacement
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn is_in_progress(&self) -> bool {
        self.in_progress
    }

    pub fn update_draginfo(&mut self, rl: &RaylibHandle, camera: &Camera2D) {
        if self.drag_started(rl) {
            self.started = true;
            self.screen_displacement.init(rl.get_mouse_position());
            self.in_progress = true;
        } else {
            self.started = false;
        }

        self.screen_displacement.set_current_position(rl.get_mouse_position());

        self.world_displacement.init(rl.get_screen_to_world2D(self.screen_displacement.starting_pos,camera));
        self.world_displacement.set_current_position(rl.get_screen_to_world2D(self.screen_displacement.current_pos,camera));

        if self.drag_ended(rl) {
            self.done = true;
            self.in_progress = false;
        } else {
            self.done = false;
        }
    }

    fn drag_started(&self, d: &RaylibHandle) -> bool {
        let middle_down = d.is_mouse_button_down(self.button_type);
        let space_down = d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE);
        let middle_pressed = d.is_mouse_button_pressed(self.button_type);
        let space_pressed = d.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE);

        if self.use_space {
            (middle_pressed && !space_down) || (space_pressed && !middle_down)
        } else {
            middle_pressed
        }
    }


    fn drag_ended(&self, d: &RaylibHandle) -> bool {
        let middle_released = d.is_mouse_button_released(self.button_type);
        let space_released = d.is_key_released(raylib::consts::KeyboardKey::KEY_SPACE);
        let middle_down = d.is_mouse_button_down(self.button_type);
        let space_down = d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE);

        if self.use_space {
            (middle_released && !space_down) || (space_released && !middle_down)
        } else {
            middle_released
        }
    }
}

