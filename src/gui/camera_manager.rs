use raylib::camera::Camera2D;
use raylib::math::Vector2;
use raylib::RaylibHandle;

use crate::gui::drag_information::DragInfo;

#[derive(Default)]
pub struct CameraManager {
    pub starting_target: Vector2,
    pub starting_position: Vector2,
}

impl CameraManager {
    pub fn handle_camera(&mut self, d: &RaylibHandle, camera: &mut Camera2D, drag_info:&DragInfo) {
        handle_mouse_wheel(d, camera);


        if drag_info.is_started() {
            self.starting_target = camera.target;
        }

        if drag_info.is_in_progress() {
            camera.target.x = self.starting_target.x + drag_info.world_displacement.delta.x;
            camera.target.y = self.starting_target.y + drag_info.world_displacement.delta.y;
        }
    }
}


fn handle_mouse_wheel(rl: &RaylibHandle, camera: &mut Camera2D) {
    let wheel = rl.get_mouse_wheel_move();

    if wheel.abs() <= 0.0 {
        return;
    }

    let old_zoom = camera.zoom;
    let zoom = (old_zoom * 1.2_f32.powf(wheel as f32)).clamp(0.1, 4.0);

    let zoom_factor = zoom / old_zoom;
    let mouse_position = rl.get_screen_to_world2D(rl.get_mouse_position(), *camera);

    camera.target.x = (camera.target.x - mouse_position.x) / zoom_factor + mouse_position.x;
    camera.target.y = (camera.target.y - mouse_position.y) / zoom_factor + mouse_position.y;
    camera.zoom = zoom;
}

