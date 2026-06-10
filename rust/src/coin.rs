use godot::classes::{IRigidBody3D, Material, MeshInstance3D, RigidBody3D, StandardMaterial3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct Coin {
    base: Base<RigidBody3D>,
    owner: Option<CoinOwners>,
    #[export]
    coin_body_mesh: Option<Gd<MeshInstance3D>>,
}

enum CoinOwners {
    PLAYER,
    NPC,
}

#[godot_api]
impl IRigidBody3D for Coin {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            base,
            owner: None,
            coin_body_mesh: None,
        }
    }

    fn process(&mut self, delta: f32) {
        if self.base().is_sleeping() {
            return;
        }
        return;
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

        let arc_up = Vector3::UP * strength * 0.9;
        let forward = dir * strength;
        let launch_velocity = forward + arc_up;

        self.base_mut().set_linear_velocity(launch_velocity);

        let flip_torque = dir.cross(Vector3::UP).normalized() * 2.0 * 8.0;
        self.base_mut().set_angular_velocity(flip_torque);
    }
    #[func]
    pub fn set_owner(&mut self, player: bool) {
        if player {
            self.owner = Some(CoinOwners::PLAYER);
        } else {
            self.owner = Some(CoinOwners::NPC);
        }
        self.set_color_based_on_owner();
    }
    #[func]
    pub fn set_color_based_on_owner(&mut self) {
        let Some(owner) = &self.owner else {
            godot_print!("no owner, cant set color");
            return;
        };

        match owner {
            CoinOwners::NPC => {
                if let Some(body_mesh) = &mut self.coin_body_mesh {
                    let material = body_mesh
                        .get_surface_override_material(0)
                        .unwrap()
                        .cast::<StandardMaterial3D>();

                    let mut unique_mat = material.duplicate_resource();
                    unique_mat.set_albedo(Color::from_rgb(1.0, 0.0, 0.0));
                    let mat: Gd<Material> = unique_mat.upcast();
                    body_mesh.set_surface_override_material(0, &mat);
                } else {
                    godot_error!("no coin body selected, on coin");
                }
            }
            CoinOwners::PLAYER => {
                return;
            }
        }
    }
}
