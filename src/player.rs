use std::f32::consts::PI;

use gdnative::{
    api::{InputEventMouseMotion, Position3D},
    prelude::*,
};

use crate::{clamp, OptRef};

const CAMERA_MOUSE_SPEED: f32 = 0.001;
const CAMERA_CONTROLLER_SPEED: f32 = 0.1;
const DEG_TO_RAD: f32 = PI / 180.0f32;
const CAMERA_X_ROT_MIN: f32 = -89.9 * DEG_TO_RAD;
const CAMERA_X_ROT_MAX: f32 = 70.0 * DEG_TO_RAD;

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
    player_pivot: OptRef<Spatial>,
    cam_x_rot: f32,
    cam_pivot: OptRef<Position3D>,
    cam_pivot_x: OptRef<Position3D>,
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
            player_pivot: OptRef::None,
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
        self.player_pivot = OptRef::from_node(base, "Pivot");
        self.cam_pivot = OptRef::from_node(base, "CameraPivot");
        self.cam_pivot_x = OptRef::from_node(base, "CameraPivot/cam_x_rot");
        // init x_rot according to scene setup
        self.cam_x_rot = -self.cam_pivot_x.tref().rotation().x;
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
        self.process_input();
        self.velocity.x = self.motion.x;
        self.velocity.z = self.motion.y;
        self.velocity.y -= self.fall_acceleration;

        // or last parameter `false` to interact with RigidBody?
        self.velocity = base.move_and_slide(
            self.velocity * self.speed,
            Vector3::UP,
            false,
            4,
            0.785398,
            true,
        );

        // orient player in the motion Vector2 direction
        if self.motion.length() > 0.1 {
            let angle = Vector2::UP.angle_to(self.motion);
            let basis = Basis::IDENTITY.rotated(Vector3::UP, -angle);
            let mut tr = self.player_pivot.tref().transform();
            tr.basis = basis.orthonormalized();
            self.player_pivot.set_transform(tr);
        }
    }

    fn process_input(&mut self) {
        let input = Input::godot_singleton();

        // movement
        self.motion =
            input.get_vector("move_left", "move_right", "move_forward", "move_back", -1.0);
        // adjust length only if it goes over 1.0 to allow walking
        if self.motion.length() > 1.0 {
            self.motion = self.motion.normalized();
        }

        // controller camera
        let look = input.get_vector("look_left", "look_right", "look_up", "look_down", -1.0);
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
            CAMERA_X_ROT_MIN,
            CAMERA_X_ROT_MAX,
        );
        self.cam_pivot_x
            .tref()
            .set_rotation(Vector3::new(-self.cam_x_rot, 0.0, 0.0));
    }
}
