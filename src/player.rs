use std::f32::consts::PI;

use gdnative::{
    api::{InputEventMouseMotion, SpringArm},
    globalscope::*,
    prelude::*,
};

use crate::{clamp, OptRef};

const CAMERA_MOUSE_SPEED: f32 = 0.001;
const CAMERA_CONTROLLER_SPEED: f32 = 0.1;
const LERP_VAL: f32 = 0.15;
// const DEG_TO_RAD: f32 = PI / 180.0f32;
// const CAMERA_X_ROT_MIN: f32 = -89.9 * DEG_TO_RAD;
// const CAMERA_X_ROT_MAX: f32 = 70.0 * DEG_TO_RAD;

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
    player_pivot: OptRef<Spatial>,
    cam_x_rot: f32,
    cam_pivot: OptRef<Spatial>,
    cam_arm_x: OptRef<SpringArm>,
}

#[methods]
impl Player {
    fn new(_base: &KinematicBody) -> Self {
        Player {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            velocity: Vector3::ZERO,
            player_pivot: OptRef::None,
            cam_x_rot: 0.0,
            cam_pivot: OptRef::None,
            cam_arm_x: OptRef::None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &KinematicBody) {
        base.upcast::<Node>().print_tree_pretty();
        self.player_pivot = OptRef::from_node(base, "Pivot");
        self.cam_pivot = OptRef::from_node(base, "CameraPivot");
        self.cam_arm_x = OptRef::from_node(base, "CameraPivot/cam_x_rot");
        // init x_rot according to scene setup
        self.cam_x_rot = -self.cam_arm_x.tref().rotation().x;
        Input::godot_singleton().set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
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
    fn _physics_process(&mut self, #[base] base: &KinematicBody, delta: f64) {
        self.process_input(base);

        // or last parameter `false` to interact with RigidBody?
        if !base.is_on_floor() || self.velocity.length_squared() > 0.0 {
            // apply gravity and move
            self.velocity.y -= self.fall_acceleration * (delta as f32);
            self.velocity =
                base.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.785398, true);
        }
    }
}

impl Player {
    fn process_input(&mut self, base: &KinematicBody) {
        let input = Input::godot_singleton();

        // movement ... for whatever reason the `get_vector` do a small pause when hit WSAD :/
        let inp = input.get_vector("move_left", "move_right", "move_forward", "move_back", -1.0);
        let direction = base.transform().basis * Vector3::new(inp.x, 0.0, inp.y);

        // handle camera
        let look = input.get_vector("look_left", "look_right", "look_up", "look_down", -1.0);
        self.rotate_cam(look * CAMERA_CONTROLLER_SPEED);

        // handle jump
        if base.is_on_floor() && input.is_action_just_pressed("jump", false) {
            self.velocity.y += self.jump_impulse;
        }

        // velocity is used for movement
        self.update_velocity(direction);
    }

    fn update_velocity(&mut self, direction: Vector3) {
        let mut direction = direction;
        // adjust movement according to camera basis
        let dir_len = direction.length();
        if dir_len > 0.0 {
            // adjust direction according to camera if it exists
            if let OptRef::Some(_) = self.cam_pivot {
                direction = direction.rotated(Vector3::UP, self.cam_pivot.tref().rotation().y);
            }
            if dir_len > 1.0 {
                direction = direction.normalized();
            }
            // apply speed
            self.velocity.x = direction.x * self.speed;
            self.velocity.z = direction.z * self.speed;

            // orient player in the direction of the velocity
            let mut player_rot = self.player_pivot.tref().rotation();
            player_rot.y = lerp_angle(
                player_rot.y..(-self.velocity.x).atan2(-self.velocity.z),
                LERP_VAL,
            );
            self.player_pivot.tref().set_rotation(player_rot);
        } else {
            self.velocity.x = move_toward(self.velocity.x..=0.0, self.speed);
            self.velocity.z = move_toward(self.velocity.z..=0.0, self.speed);
        }
    }

    fn rotate_cam(&mut self, look: Vector2) {
        // if no camera, just finish
        if let OptRef::None = self.cam_pivot {
            return;
        }

        // process camera rotation
        let cam = self.cam_pivot.tref();
        // rotate left/right
        if look.x != 0.0 {
            cam.rotate_y(-look.x as f64);
        }

        // compute how much to rotate
        self.cam_x_rot += look.y;
        self.cam_x_rot = clamp(self.cam_x_rot, -PI / 4.0, PI / 4.0);
        self.cam_arm_x
            .tref()
            .set_rotation(Vector3::new(-self.cam_x_rot, 0.0, 0.0));
    }
}
