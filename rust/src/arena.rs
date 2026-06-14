use std::collections::HashMap;

use godot::classes::{Area3D, INode3D, Node3D, Timer};
use godot::prelude::*;

use crate::coin::Coin;
use crate::player::Player;
use crate::player::player_movement::HandlePlayerInput;

const MAX_THROWS: i32 = 5;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Arena {
    base: Base<Node3D>,
    throws_done: i32,
    #[export]
    arena_wall_pointer: Option<Gd<Node3D>>,
    #[export]
    sleeping_coins_timer: Option<Gd<Timer>>,
    #[export]
    player_arena_area_enter: Option<Gd<Area3D>>,
    #[export]
    coin_loc_parent: Option<Gd<Node3D>>,
}

#[godot_api]
impl INode3D for Arena {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            throws_done: 0,
            arena_wall_pointer: None,
            sleeping_coins_timer: None,
            player_arena_area_enter: None,
            coin_loc_parent: None,
        }
    }

    fn ready(&mut self) {
        if let Some(area) = &mut self.player_arena_area_enter.clone() {
            let callable = self.base().callable("player_entered_start_area");
            area.connect("body_entered", &callable);
        }

        if let Some(timer) = &mut self.sleeping_coins_timer.clone() {
            let callable = self.base().callable("timer_popped");
            timer.connect("timeout", &callable);
        }
    }
}

#[godot_api]
impl Arena {
    #[func]
    fn player_entered_start_area(&mut self, body: Gd<Node3D>) {
        godot_print!("in area");

        if !body.is_in_group("player") {
            return;
        }

        self.set_player_position(body);
        self.set_timer_state(true);

        self.throws_done = 0;
    }

    fn set_player_position(&mut self, body: Gd<Node3D>) {
        let Some(area) = &self.player_arena_area_enter else {
            return;
        };

        let Ok(mut player) = body.try_cast::<Player>() else {
            return;
        };

        let mut player_ref = player.bind_mut();

        let area_glob_pos = area.get_global_position();
        player_ref.base_mut().set_global_position(area_glob_pos);
        player_ref.set_player_locked_in_place(true);
    }

    fn set_timer_state(&mut self, state: bool) {
        let Some(timer) = &mut self.sleeping_coins_timer else {
            return;
        };

        match state {
            true => timer.start(),
            false => timer.stop(),
        }
    }

    #[func]
    fn timer_popped(&mut self) {
        let Some(coin_parent) = &mut self.coin_loc_parent else {
            return;
        };

        let coins = coin_parent.get_children();
        let mut all_sleeping = false;
        let mut hmap = HashMap::new();

        for c in coins.iter_shared() {
            if !c.is_in_group("coin") {
                continue;
            }

            let Ok(coin) = c.try_cast::<Coin>() else {
                continue;
            };

            godot_print!("before sleep {}", all_sleeping);
            if coin.bind().base().is_sleeping() {
                all_sleeping = true;
            } else {
                all_sleeping = false;
                break;
            }

            let distance = {
                let coin_ref = coin.bind();
                self.get_distance_from_wall(coin_ref.base().get_global_position())
            };

            hmap.insert(coin, distance);
        }

        if !all_sleeping {
            return;
        }

        let mut lowest_c: Option<Gd<Coin>> = None;
        let mut lowest_d: f32 = f32::MAX;
        for (c, d) in &hmap {
            if *d > lowest_d {
                continue;
            }
            lowest_d = *d;
            lowest_c = Some(c.clone());
        }

        if let Some(mut coin) = lowest_c {
            let owner: String = coin.bind_mut().get_owner_as_string();
            godot_print!("lowest is {} with distance {}", owner, lowest_d.to_string());
        }
    }

    fn get_distance_from_wall(&mut self, coin_pos: Vector3) -> f32 {
        let Some(wall) = &mut self.arena_wall_pointer else {
            godot_error!("no wall setup");
            return 0.0;
        };

        let wall_pos = wall.get_global_position();
        let wall_normal = -wall.get_global_transform().basis.col_c();

        let distance = (coin_pos - wall_pos).dot(wall_normal).abs();

        return distance;
    }
}
