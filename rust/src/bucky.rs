use godot::classes::{Camera3D, CharacterBody3D, ICharacterBody3D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Bucky {
    base: Base<CharacterBody3D>,
    movement_dir: Vector3,
    #[export]
    camera_base: Option<Gd<Camera3D>>,
    #[export]
    char_body_base: Option<Gd<Node3D>>,
}

#[godot_api]
impl ICharacterBody3D for Bucky {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            movement_dir: Vector3::ZERO,
            camera_base: None,
            char_body_base: None,
        }
    }

    fn process(&mut self, delta: f32) {
        self.handle_input();
        self.handle_movement();
    }
}

impl Bucky {
    fn handle_input(&mut self) {
        let input = Input::singleton();
        let mut dir = Vector3::ZERO;

        let Some(cam) = &self.camera_base else {
            return;
        };

        let basis = cam.get_global_transform().basis;
        let forward = -basis.col_c();
        let right = basis.col_a();

        if input.is_action_pressed("forward") {
            dir += forward;
        }
        if input.is_action_pressed("backwards") {
            dir -= forward;
        }
        if input.is_action_pressed("left_rot") {
            dir -= right;
        }
        if input.is_action_pressed("right_rot") {
            dir += right;
        }

        dir.y = 0.0;

        if dir.length() > 0.0 {
            dir = dir.normalized();
        }

        self.movement_dir = dir;
        self.turn_body_towards_dir();
    }

    fn turn_body_towards_dir(&mut self) {
        let Some(char_body) = &mut self.char_body_base else {
            return;
        };
        if self.movement_dir == Vector3::ZERO {
            return;
        }
        let current_pos = char_body.get_global_position();
        let target_pos = current_pos + self.movement_dir;
        char_body.look_at(target_pos);
    }

    fn handle_movement(&mut self) {
        let mut velocity = self.base().get_velocity();

        velocity.x = self.movement_dir.x * 5.0;
        velocity.z = self.movement_dir.z * 5.0;

        if !self.base().is_on_floor() {
            velocity.y += -5.0;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
