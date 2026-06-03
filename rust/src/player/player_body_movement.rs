use crate::player::Player;
use godot::prelude::*;

pub trait HandlePlayerBodyMovement {
    fn rotate_upper_body_with_view(&mut self);
}

impl HandlePlayerBodyMovement for Player {
    fn rotate_upper_body_with_view(&mut self) {
        let cam_euler = self
            .player_body
            .player_camera_base
            .get_global_basis()
            .get_euler();

        self.player_body
            .player_upper_body
            .set_global_rotation(Vector3 {
                x: (0.0),
                y: (cam_euler.y),
                z: (0.0),
            });
    }
}
