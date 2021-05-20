use raylib::RaylibHandle;
use raylib::camera::Camera2D;
use raylib::consts::KeyboardKey;

pub fn handle_camera(d :&RaylibHandle, camera:&mut Camera2D) {
    handle_mouse_wheel(d,camera);

    let space = d.is_key_pressed(KeyboardKey::KEY_SPACE);

}

fn handle_mouse_wheel(rl:&RaylibHandle, camera:&mut Camera2D) {
    let wheel = rl.get_mouse_wheel_move();

    camera.zoom += wheel*0.1;
    camera.zoom = camera.zoom.clamp(0.1,2.0)
}