use godot::classes::RigidBody3D;
use godot::prelude::*;

use crate::coin::Coin;
use crate::player::{Player, player_coin_controller};

pub trait HandlePlayerCoinInput {
    fn spawn_coin_in_hand(&mut self);
}

impl HandlePlayerCoinInput for Player {
    fn spawn_coin_in_hand(&mut self) {
        let (Some(coin_spot), Some(coin_scene)) =
            (&mut self.player_coin_spot, &self.player_coin_model)
        else {
            godot_print!("coin spot not linked");
            return;
        };

        let mut coin = coin_scene.instantiate_as::<Coin>();

        coin_spot.add_child(&coin.upcast::<Node>());
        godot_print!("spawned");
    }
}
