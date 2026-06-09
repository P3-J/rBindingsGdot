use godot::classes::{IRigidBody3D, RigidBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct Coin {
    base: Base<RigidBody3D>,
}

#[godot_api]
impl IRigidBody3D for Coin {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self { base }
    }

    fn process(&mut self, delta: f32) {
        if self.base().is_sleeping() {
            return;
        }

        let scene = self.base_mut().get_tree().get_current_scene().unwrap();
        let wall = scene.get_node_as::<Node3D>("arena1/arena1_wall");

        let wall_pos = wall.get_global_position();
        let wall_normal = -wall.get_global_transform().basis.col_c();

        let coin_pos = self.base().get_global_position();
        let distance = (coin_pos - wall_pos).dot(wall_normal).abs();

        godot_print!("{}", distance);
    }
}

#[godot_api]
impl Coin {
    #[func]
    pub fn launch_coin(&mut self, dir: Vector3, strength: f32) {
        self.base_mut().set_sleeping(false);
        self.base_mut().set_lock_rotation_enabled(false);
        self.base_mut().set_freeze_enabled(false);

        let dir = dir.normalized();

        let arc_up = Vector3::UP * strength * 0.6;
        let forward = dir * strength;
        let launch_velocity = forward + arc_up;

        self.base_mut().set_linear_velocity(launch_velocity);

        let flip_torque = dir.cross(Vector3::UP).normalized() * 2.0 * 8.0;
        self.base_mut().set_angular_velocity(flip_torque);
    }
}
