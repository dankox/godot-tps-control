use gdnative::{
    api::{InputEventMouseMotion, Position3D},
    prelude::*,
};

use crate::{clamp, OptRef};

const CAMERA_MOUSE_SPEED: f32 = 0.001;
const CAMERA_CONTROLLER_SPEED: f32 = 0.1;
const CAMERA_X_ROT_MIN: f32 = -89.9;
const CAMERA_X_ROT_MAX: f32 = 70.0;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    #[property(default = 14.0)]
    speed: f32,
    #[property(default = 75.0)]
    fall_acceleration: f32,
    #[property(default = 20.0)]
    jump_impulse: f32,

    velocity: Vector3,
    motion: Vector2,
    cam_x_rot: f32,
    cam_pivot: OptRef<Position3D>,
    cam_pivot_x: OptRef<Position3D>,
    // camera: OptRef<Camera>,
}

#[methods]
impl Player {
    fn new(_base: &KinematicBody) -> Self {
        Player {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            velocity: Vector3::ZERO,
            motion: Vector2::ZERO,
            cam_x_rot: 0.0,
            cam_pivot: OptRef::None,
            cam_pivot_x: OptRef::None,
        }
    }

    #[method]
    fn _init(&mut self) {
        let input = Input::godot_singleton();
        input.set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
    }

    #[method]
    fn _ready(&mut self, #[base] base: &KinematicBody) {
        base.upcast::<Node>().print_tree_pretty();
        // self.cam_pivot = OptRef::Some(get_node!(base, "CameraPivot", Position3D));
        self.cam_pivot = OptRef::from_node(base, "CameraPivot");
        self.cam_pivot_x = OptRef::from_node(base, "CameraPivot/cam_x_rot");
    }

    #[method]
    fn _input(&mut self, #[base] _base: &KinematicBody, event: Ref<InputEvent>) {
        // camera handling by mouse movement
        match event.try_cast::<InputEventMouseMotion>() {
            Ok(e) => unsafe {
                self.rotate_cam(e.assume_safe().relative() * CAMERA_MOUSE_SPEED);
            },
            Err(_) => {}
        };
    }

    #[method]
    fn _physics_process(&mut self, #[base] base: &KinematicBody, _delta: f64) {
        // godot_print!("delta: {}", delta);
        self.process_input(base);
        self.velocity.y -= self.fall_acceleration * self.speed;

        self.velocity = base.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.785398, true);
        // or last parameter `false` to interact with RigidBody?
    }

    fn process_input(&mut self, _base: &KinematicBody) {
        let mut look = Vector2::ZERO;
        let input = Input::godot_singleton();

        // movement
        self.motion.x = (input.get_action_strength("move_right", false)
            - input.get_action_strength("move_left", false)) as f32;
        self.motion.y = (input.get_action_strength("move_back", false)
            - input.get_action_strength("move_forward", false)) as f32;
        // adjust length only if it goes over 1.0 to allow walking
        if self.motion.length() > 1.0 {
            self.motion = self.motion.normalized();
            godot_print!("motion {}x{}", self.motion.x, self.motion.y);
        }
        self.velocity.x = self.motion.x * self.speed;
        self.velocity.z = self.motion.y * self.speed;

        // controller camera
        look.x = (input.get_action_strength("look_right", false)
            - input.get_action_strength("look_left", false)) as f32;
        look.y = (input.get_action_strength("look_down", false)
            - input.get_action_strength("look_up", false)) as f32;
        if look.length_squared() > 0.0 {
            look = look.normalized()
        }
        self.rotate_cam(look * CAMERA_CONTROLLER_SPEED);
    }

    fn rotate_cam(&mut self, look: Vector2) {
        // if no camera, just finish
        if let OptRef::None = self.cam_pivot {
            return;
        }

        // process camera rotation
        let cam = self.cam_pivot.tref();
        // rotate left/right
        cam.rotate_y(-look.x as f64);

        // compute how much to rotate
        self.cam_x_rot += look.y;
        self.cam_x_rot = clamp(
            self.cam_x_rot,
            CAMERA_X_ROT_MIN.to_radians(),
            CAMERA_X_ROT_MAX.to_radians(),
        );
        self.cam_pivot_x
            .tref()
            .set_rotation(Vector3::new(-self.cam_x_rot, 0.0, 0.0));
    }
}
