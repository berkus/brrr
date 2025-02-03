//
// DELETE WHOLE FILE
//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use cgmath::*; //, glium::glutin};

// @todo ❌ this converts to an entity with some components and a system to process movement
pub struct CameraState {
    aspect_ratio: f32,
    fov: f32,
    zfar: f32,
    znear: f32,
    position: Point3<f32>,
    direction: Vector3<f32>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    rotating_left: bool,
    rotating_right: bool,
}

impl Default for CameraState {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            // Keep ratio fixed at 4/3
            aspect_ratio: 4.0 / 3.0,
            fov: 90.0,
            zfar: 100.0,
            znear: 0.1,
            position: Point3 {
                x: 0.1,
                y: 0.1,
                z: 1.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            rotating_left: false,
            rotating_right: false,
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = Point3::from(pos);
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = Vector3::from(dir);
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect_ratio = aspect;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        perspective(Deg(self.fov), self.aspect_ratio, self.znear, self.zfar).into()
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let up = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        Matrix4::look_at_rh(self.position, Point3::from_vec(self.direction), up).into()
    }

    pub fn update(&mut self) {
        let f = self.direction.normalize();
        let up = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        if self.moving_up {
            self.position.x += u.x * 0.01;
            self.position.y += u.y * 0.01;
            self.position.z += u.z * 0.01;
        }

        if self.moving_left {
            self.position.x -= s.x * 0.01;
            self.position.y -= s.y * 0.01;
            self.position.z -= s.z * 0.01;
        }

        if self.moving_down {
            self.position.x -= u.x * 0.01;
            self.position.y -= u.y * 0.01;
            self.position.z -= u.z * 0.01;
        }

        if self.moving_right {
            self.position.x += s.x * 0.01;
            self.position.y += s.y * 0.01;
            self.position.z += s.z * 0.01;
        }

        if self.moving_forward {
            self.position.x += f.x * 0.01;
            self.position.y += f.y * 0.01;
            self.position.z += f.z * 0.01;
        }

        if self.moving_backward {
            self.position.x -= f.x * 0.01;
            self.position.y -= f.y * 0.01;
            self.position.z -= f.z * 0.01;
        }

        if self.rotating_left {
            self.direction = Quaternion::from_angle_y(Deg(1.0)).rotate_vector(self.direction);
        }

        if self.rotating_right {
            self.direction = Quaternion::from_angle_y(Deg(-1.0)).rotate_vector(self.direction);
        }

        // trace!("Camera pos {:?} dir {:?}", self.position, self.direction);
    }

    // pub fn process_input(&mut self, event: &glutin::event::WindowEvent) {
    //     let input = match *event {
    //         glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
    //         _ => return,
    //     };
    //     let pressed = input.state == glutin::event::ElementState::Pressed;
    //     let key = match input.virtual_keycode {
    //         Some(key) => key,
    //         None => return,
    //     };
    //     match key {
    //         glutin::event::VirtualKeyCode::Up => self.moving_up = pressed,
    //         glutin::event::VirtualKeyCode::Down => self.moving_down = pressed,
    //         glutin::event::VirtualKeyCode::Left => self.rotating_left = pressed,
    //         glutin::event::VirtualKeyCode::Right => self.rotating_right = pressed,
    //         glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
    //         glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
    //         glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
    //         glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
    //         _ => (),
    //     };
    // }
}
