use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    base: Base<Sprite2D>,
    angular_speed: f64,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }

    fn ready(&mut self) {
        self.base_mut().call_deferred("scream_hello", &[]);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn scream_hello(&mut self) {
        godot_print!("scream_hello called");
        let mut ebus = self.base().get_node_as::<Node>("/root/EventBus");
        ebus.emit_signal("scream_hello", &["hello".to_variant()]);
    }
}
