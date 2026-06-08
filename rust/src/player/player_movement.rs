use crate::player::Player;
use godot::classes::{Input, InputEvent, InputEventMouseMotion};
use godot::global::deg_to_rad;
use godot::prelude::*;

const GRAVITY: f32 = -3.8;
const MOVEMENT_SPEED: f32 = 10.0;

pub struct PlayerInputCollection {
    pub dir: Vector3,
}

impl Default for PlayerInputCollection {
    fn default() -> Self {
        Self { dir: Vector3::ZERO }
    }
}

pub trait HandlePlayerInput {
    fn handle_input(&mut self);
    fn handle_movement(&mut self);
    fn handle_camera_input(&mut self, event: Gd<InputEvent>);
}

impl HandlePlayerInput for Player {
    fn handle_input(&mut self) {
        let input = Input::singleton();
        let mut dir = Vector3::ZERO;

        let basis = self.base().get_transform().basis;
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

        if dir.length() > 0.0 {
            dir = dir.normalized();
        }

        self.player_movement_col.dir = dir;
    }

    fn handle_movement(&mut self) {
        let mut velocity = self.base().get_velocity();

        velocity.x = self.player_movement_col.dir.x * MOVEMENT_SPEED;
        velocity.z = self.player_movement_col.dir.z * MOVEMENT_SPEED;

        if !self.base().is_on_floor() {
            velocity.y += if velocity.y > -5.0 { GRAVITY } else { 0.0 };
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }

    fn handle_camera_input(&mut self, event: Gd<InputEvent>) {
        if !self.player_body.player_camera_base.is_node_ready() {
            return;
        }

        if let Ok(mouse_event) = event.try_cast::<InputEventMouseMotion>() {
            let delta = mouse_event.get_relative();

            self.base_mut().rotate_y(-delta.x as f32 * 0.005);

            let mut cam_rot = self.player_body.player_camera_base.get_rotation();
            cam_rot.x -= delta.y * 0.005;
            cam_rot.x = cam_rot
                .x
                .clamp(deg_to_rad(-80.0) as f32, deg_to_rad(80.0) as f32);
            self.player_body.player_camera_base.set_rotation(cam_rot);
        }
    }
}
