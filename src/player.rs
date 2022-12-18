use gdnative::{api::InputEventMouseMotion, prelude::*};

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
}

#[methods]
impl Player {
    fn new(_base: &KinematicBody) -> Self {
        Player {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            velocity: Vector3::ZERO,
        }
    }

    fn process_input(&mut self, base: &KinematicBody) {
        let mut direction = Vector2::ZERO;
        let mut look = Vector2::ZERO;
        let input = Input::godot_singleton();

        // movement
        direction.x = (input.get_action_strength("move_right", false)
            - input.get_action_strength("move_left", false)) as f32;
        direction.y = (input.get_action_strength("move_back", false)
            - input.get_action_strength("move_forward", false)) as f32;
        if direction.length_squared() > 0.0 {
            direction = direction.normalized()
        }
        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.y * self.speed;
        // camera
        look.x = (input.get_action_strength("look_right", false)
            - input.get_action_strength("look_left", false)) as f32;
        look.y = (input.get_action_strength("look_down", false)
            - input.get_action_strength("look_up", false)) as f32;
        if look.length_squared() > 0.0 {
            look = look.normalized()
        }
        self.rotate_self(base, look);
    }

    fn rotate_self(&mut self, base: &KinematicBody, look: Vector2) {
        // process the rotation
        base.rotate(Vector3::UP, -look.x as f64);
        base.rotate(Vector3::LEFT, look.y as f64);
    }

    #[method]
    fn _input(&mut self, #[base] base: &KinematicBody, event: Ref<InputEvent>) {
        // let e =
        match event.try_cast::<InputEventMouseMotion>() {
            Ok(e) => unsafe {
                self.rotate_self(base, e.assume_safe().relative() * 0.001);
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
}
