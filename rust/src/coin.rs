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
}
