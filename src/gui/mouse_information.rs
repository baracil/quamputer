use raylib::camera::Camera2D;

use raylib::math::Vector2;
use raylib::RaylibHandle;
use crate::gui::drag_information::DragInfo;

#[derive(Clone)]
pub struct MouseInformation {
    pub screen_pos: Vector2,
    pub world_pos: Vector2,
    pub left_drag: DragInfo,
    pub middle_drag: DragInfo,
    pub right_drag: DragInfo,
}



impl MouseInformation {
    pub fn new() -> Self {
        return MouseInformation {
            world_pos: Default::default(),
            screen_pos: Default::default(),
            left_drag: DragInfo::new(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON,false),
            middle_drag: DragInfo::new(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON,true),
            right_drag: DragInfo::new(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON,false),
        };
    }

    pub fn update(&mut self, rl: &RaylibHandle, camera: &Camera2D) {
        self.screen_pos = rl.get_mouse_position();
        self.world_pos = rl.get_screen_to_world2D(self.screen_pos, camera);
        self.left_drag.update_draginfo(rl,camera);
        self.middle_drag.update_draginfo(rl,camera);
        self.right_drag.update_draginfo(rl,camera);
    }
}