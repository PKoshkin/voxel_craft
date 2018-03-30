use game_application::glutin;
use game_application::cgmath::{Vector3, Point3};
use game_application::geometry::get_normalized;


pub struct Camera {
    pub position: Point3<f32>,
    aspect_ratio: f32,
    forward_direction: Vector3<f32>,
    up_direction: Vector3<f32>,

    cursor_position: (f32, f32),
    know_cursor: bool,

    move_speed: f32,
    rotate_speed: f32,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    rotate_clockwise: bool,
    rotate_counterclockwise: bool,

    cursor_move: (f32, f32)
}


impl Camera {
    pub fn new(aspect_ratio: f32, position: (f32, f32, f32), direction: (f32, f32, f32)) -> Camera {
        Camera {
            position: Point3::new(position.0, position.1, position.2),
            aspect_ratio: aspect_ratio,
            forward_direction: Vector3::new(direction.0, direction.1, direction.2),
            up_direction: Vector3::new(0.0, 1.0, 0.0),

            cursor_position: (0.0, 0.0),
            know_cursor: false,

            move_speed: 0.01,
            rotate_speed: 0.01,

            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            rotate_clockwise: false,
            rotate_counterclockwise: false,

            cursor_move: (0.0, 0.0)
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.001;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0,                                    0.0, 0.0],
            [                  0.0,   f,                                    0.0, 0.0],
            [                  0.0, 0.0,        (zfar + znear) / (zfar - znear), 1.0],
            [                  0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let right_direction = self.forward_direction.cross(self.up_direction);
        let p = (-self.position.x * right_direction.x - self.position.y * right_direction.y - self.position.z * right_direction.z,
                 -self.position.x * self.up_direction.x - self.position.y * self.up_direction.y - self.position.z * self.up_direction.z,
                 -self.position.x * self.forward_direction.x - self.position.y * self.forward_direction.y - self.position.z * self.forward_direction.z);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [right_direction.x, self.up_direction.x, self.forward_direction.x, 0.0],
            [right_direction.y, self.up_direction.y, self.forward_direction.y, 0.0],
            [right_direction.z, self.up_direction.z, self.forward_direction.z, 0.0],
            [              p.0,                 p.1,                      p.2, 1.0],
        ]
    }

    pub fn update(&mut self) {
        let mut right_direction = self.forward_direction.cross(self.up_direction);

        if self.moving_up {
            self.position += self.move_speed * self.up_direction;
        }
        if self.moving_down {
            self.position -= self.move_speed * self.up_direction;
        }
        if self.moving_left {
            self.position -= self.move_speed * right_direction;
        }
        if self.moving_right {
            self.position += self.move_speed * right_direction;
        }
        if self.moving_forward {
            self.position += self.move_speed * self.forward_direction;
        }
        if self.moving_backward {
            self.position -= self.move_speed * self.forward_direction;
        }

        if self.know_cursor {
            self.forward_direction += self.rotate_speed * self.cursor_move.0 * right_direction;
            self.forward_direction -= self.rotate_speed * self.cursor_move.1 * self.up_direction;
            self.forward_direction = get_normalized(self.forward_direction);
            self.up_direction = right_direction.cross(self.forward_direction);
        }

        right_direction = self.forward_direction.cross(self.up_direction);
        if self.rotate_clockwise {
            self.up_direction += self.rotate_speed * right_direction;
            self.up_direction = get_normalized(self.up_direction);
        }
        if self.rotate_counterclockwise {
            self.up_direction -= self.rotate_speed * right_direction;
            self.up_direction = get_normalized(self.up_direction);
        }

        self.cursor_move = (0.0, 0.0);
    }

    pub fn handle_event(&mut self, event: &glutin::WindowEvent) {
        if let glutin::WindowEvent::CursorMoved{position, ..} = *event {
            if self.know_cursor {
                self.cursor_move = (position.0 as f32 - self.cursor_position.0, position.1 as f32 - self.cursor_position.1);
            } else {
                self.know_cursor = true;
            }
            self.cursor_position.0 = position.0 as f32;
            self.cursor_position.1 = position.1 as f32;
        }
        if let glutin::WindowEvent::KeyboardInput{input, ..} = *event {
            let pressed = input.state == glutin::ElementState::Pressed;
            if let Some(key) = input.virtual_keycode {
                match key {
                    glutin::VirtualKeyCode::Space => self.moving_up = pressed,
                    glutin::VirtualKeyCode::LShift => self.moving_down = pressed,
                    glutin::VirtualKeyCode::A => self.moving_left = pressed,
                    glutin::VirtualKeyCode::D => self.moving_right = pressed,
                    glutin::VirtualKeyCode::W => self.moving_forward = pressed,
                    glutin::VirtualKeyCode::S => self.moving_backward = pressed,
                    glutin::VirtualKeyCode::Q => self.rotate_counterclockwise = pressed,
                    glutin::VirtualKeyCode::E => self.rotate_clockwise = pressed,
                    _ => (),
                };
            }
        }
    }
}
