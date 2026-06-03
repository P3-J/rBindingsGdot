use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent};
use godot::prelude::*;

use crate::player::player_movement::{HandlePlayerInput, PlayerInputCollection};

mod player_body_movement;
mod player_movement;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,
    player_movement_col: PlayerInputCollection,
    player_body: PlayerBodyParts,
}

struct PlayerBodyParts {
    player_camera_base: OnReady<Gd<Node3D>>,
    player_upper_body: OnReady<Gd<Node3D>>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            player_movement_col: PlayerInputCollection::default(),
            player_body: PlayerBodyParts {
                player_camera_base: OnReady::manual(),
                player_upper_body: OnReady::manual(),
            },
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.handle_input();
        self.handle_movement();
    }

    fn process(&mut self, delta: f64) {
        // for now set cam pos every frame should be done smoothly
        let s_pos = self.base().get_position();
        self.player_body.player_camera_base.set_position(s_pos);
        self.player_body.player_upper_body.set_position(s_pos);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_camera_input(event);
    }

    fn ready(&mut self) {
        self.base_mut().call_deferred("scream_hello", &[]);

        self.player_body
            .player_camera_base
            .init(self.base().get_node_as::<Node3D>("playercambase"));
        self.player_body
            .player_upper_body
            .init(self.base().get_node_as::<Node3D>("upperBody"));
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
