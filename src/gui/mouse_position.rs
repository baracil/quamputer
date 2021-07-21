use raylib::math::Vector2;
use raylib::drawing::RaylibDrawHandle;
use raylib::camera::Camera2D;
use raylib::RaylibHandle;

#[derive(Clone)]
pub struct MouseInformation {
    pub(crate) screen:Vector2,
    pub(crate) world:Vector2,
}

impl MouseInformation {

    pub fn new(rl:&RaylibHandle, camera:&Camera2D) -> Self {
        let screen = rl.get_mouse_position();
        let world = rl.get_screen_to_world2D(screen,camera);
        return MouseInformation{screen,world}
    }
}