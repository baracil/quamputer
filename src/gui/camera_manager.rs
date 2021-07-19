use raylib::RaylibHandle;
use raylib::camera::Camera2D;
use raylib::consts::KeyboardKey;
use raylib::math::Vector2;
use crate::gui::DrawingPar;
use rsgui::mouse::MouseState;


#[derive(Default)]
pub struct CameraManager {
    pub starting_target:Vector2,
    pub starting_position:Vector2,
    pub drag_info:DragInfo,
}

impl CameraManager {

    pub fn handle_camera(&mut self, d :&RaylibHandle, camera:&mut Camera2D) {
        self.drag_info.update_draginfo(d);
        handle_mouse_wheel(d,camera);


        if self.drag_info.started {
            self.starting_target = camera.target;
        }

        if self.drag_info.in_progress {
            let start = d.get_screen_to_world2D(self.drag_info.starting_position,*camera);
            let end   = d.get_screen_to_world2D(self.drag_info.current_position,*camera);
            let delta = start-end;
            camera.target.x = self.starting_target.x +delta.x;
            camera.target.y = self.starting_target.y +delta.y;
        }
    }



}

fn drag_started(d:&RaylibHandle) -> bool {
    let middle_down = d.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON);
    let space_down = d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE);
    let middle_pressed = d.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON);
    let space_pressed = d.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE);

     (middle_pressed && !space_down) || (space_pressed && !middle_down)
}


fn drag_ended(d:&RaylibHandle) -> bool {
    let middle_released = d.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON);
    let space_released = d.is_key_released(raylib::consts::KeyboardKey::KEY_SPACE);
    let middle_down = d.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON);
    let space_down = d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE);

    (middle_released && !space_down) || (space_released && !middle_down)
}


fn handle_mouse_wheel(rl:&RaylibHandle, camera:&mut Camera2D) {
    let wheel = rl.get_mouse_wheel_move();

    if wheel.abs()> 0.0 {
        camera.zoom *= 1.2_f32.powf(wheel as f32);
        camera.zoom = camera.zoom.clamp(0.1,2.0)

    }
}


#[derive(Default)]
pub struct DragInfo {
    in_progress:bool,
    started:bool,
    done:bool,
    starting_position:Vector2,
    current_position:Vector2,
    delta:Vector2,
}

impl DragInfo {

    pub fn update_draginfo(&mut self, d: &RaylibHandle) {
        if drag_started(d) {
            self.started = true;
            self.starting_position = d.get_mouse_position();
            self.in_progress = true;
        } else {
            self.started = false;
        }

        self.current_position = d.get_mouse_position();
        self.delta = self.current_position - self.starting_position;

        if drag_ended(d) {
            self.done = true;
            self.in_progress = false;
        } else {
            self.done = false;
        }
    }

}