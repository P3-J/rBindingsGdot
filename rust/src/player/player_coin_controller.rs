use godot::classes::{InputEvent, RigidBody3D};
use godot::prelude::*;

use crate::coin::{self, Coin};
use crate::player::{Player, player_coin_controller};

pub trait HandlePlayerCoinInput {
    fn spawn_coin_in_hand(&mut self);
    fn throw_coin_in_hand(&mut self, event: &Gd<InputEvent>);
}

impl HandlePlayerCoinInput for Player {
    fn spawn_coin_in_hand(&mut self) {
        let (Some(coin_spot), Some(coin_scene)) =
            (&mut self.player_coin_spot, &self.player_coin_model)
        else {
            godot_print!("coin spot not linked");
            return;
        };

        if self.player_props.coin_in_hand.is_some() {
            return;
        }

        let coin = coin_scene.instantiate_as::<Coin>();
        coin.clone().bind_mut().set_owner(true);

        coin_spot.add_child(&coin.clone().upcast::<Node>());
        self.player_props.coin_in_hand = Some(coin);

        godot_print!("spawned");
    }

    fn throw_coin_in_hand(&mut self, event: &Gd<InputEvent>) {
        if !event.is_action_pressed("throw") {
            return;
        }

        let Some(coin) = self.player_props.coin_in_hand.take() else {
            godot_print!("no coin in hand to throw");
            return;
        };

        let scene = self.base_mut().get_tree().get_current_scene().unwrap();

        coin.clone().upcast::<Node>().reparent(&scene);

        let camera_dir = self
            .player_body
            .player_camera_base
            .get_global_transform()
            .basis
            .col_c();
        coin.clone().bind_mut().launch_coin(-camera_dir, 10.0);
    }
}
