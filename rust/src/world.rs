use godot::classes::{INode3D, Node, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct World {
    base: Base<Node3D>,
    ebus: OnReady<Gd<Node>>,
}

#[godot_api]
impl INode3D for World {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            ebus: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.ebus
            .init(self.base().get_node_as::<Node>("/root/EventBus"));
        godot_print!("scream_hello2 called");
        let callable = self.base().callable("_on_player_screaming_help");
        self.ebus.connect("scream_hello", &callable);
    }
}

#[godot_api]
impl World {
    #[func]
    fn _on_player_screaming_help(&mut self, val: String) {
        godot_print!("Hello, world! {}", val);
    }
}
