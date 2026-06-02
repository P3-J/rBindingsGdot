use godot::classes::{Input, InputEvent, InputEventMouseMotion};
use godot::global::deg_to_rad;
use godot::prelude::*;

use crate::player::Player;

const GRAVITY: f32 = -3.8;

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
        let event = Input::singleton();

        if event.is_action_pressed("forward") {
            let basis_z: Vector3 = self.base().get_transform().basis.col_c();
            self.player_movement_col.dir = -basis_z;
        } else if event.is_action_pressed("backwards") {
            let basis_z: Vector3 = self.base().get_transform().basis.col_c();
            self.player_movement_col.dir = basis_z;
        } else {
            self.player_movement_col.dir = Vector3::ZERO
        }

        if event.is_action_pressed("left_rot") {
            self.base_mut().rotate_y(0.04);
        }
        if event.is_action_pressed("right_rot") {
            self.base_mut().rotate_y(-0.04);
        }
    }

    fn handle_movement(&mut self) {
        let mut velocity = self.base().get_velocity();
        velocity.z = self.player_movement_col.dir.z * 10.0;
        velocity.x = self.player_movement_col.dir.x * 10.0;

        if !self.base().is_on_floor() {
            velocity.y += if velocity.y > -5.0 { GRAVITY } else { 0.0 };
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }

    fn handle_camera_input(&mut self, event: Gd<InputEvent>) {
        if !self.player_camera_base.is_node_ready() {
            return;
        }

        if let Ok(mouse_event) = event.try_cast::<InputEventMouseMotion>() {
            let mouse_relative: Vector2 = mouse_event.get_relative();

            let mut cur_rot = self.player_camera_base.get_rotation();
            cur_rot += Vector3::new(-mouse_relative.y * 0.01, -mouse_relative.x * 0.01, 0.0);

            let x_pitch = cur_rot
                .x
                .clamp(deg_to_rad(-80.0) as f32, deg_to_rad(40.0) as f32);

            cur_rot.x = x_pitch;

            self.player_camera_base.set_rotation(cur_rot);
        };
    }
}
