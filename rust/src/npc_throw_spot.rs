use godot::classes::{INode3D, InputEvent, Node, Node3D};
use godot::prelude::*;
use rand::Rng;

use crate::coin::Coin;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct NpcThrowSpot {
    base: Base<Node3D>,

    #[export]
    coin_model: Option<Gd<PackedScene>>,
    coin_in_hand: Option<Gd<Coin>>,
}

#[godot_api]
impl INode3D for NpcThrowSpot {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            coin_model: None,
            coin_in_hand: None,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.npc_coin_throw_process(&event);
    }
}

impl NpcThrowSpot {
    pub fn npc_coin_throw_process(&mut self, ievent: &Gd<InputEvent>) {
        if !ievent.is_action_pressed("throw") {
            return;
        }

        self.spawn_coin_in_hand();
        self.throw_coin_in_hand(&ievent);
    }

    fn spawn_coin_in_hand(&mut self) {
        let Some(coin_scene) = &self.coin_model else {
            godot_print!("coin model not added for npc throw");
            return;
        };

        let coin = coin_scene.instantiate_as::<Coin>();

        self.base_mut().add_child(&coin.clone().upcast::<Node>());
        coin.clone().bind_mut().set_owner(false);
        self.coin_in_hand = Some(coin);
    }

    fn throw_coin_in_hand(&mut self, event: &Gd<InputEvent>) {
        if !event.is_action_pressed("throw") {
            return;
        }

        let Some(coin) = self.coin_in_hand.take() else {
            godot_print!("no coin in npc hand to throw");
            return;
        };

        let scene = self.base_mut().get_tree().get_current_scene().unwrap();
        let wall = scene.get_node_as::<Node3D>("arena1/coin_loc_parent");

        coin.clone().upcast::<Node>().reparent(&wall);

        let (dir, str) = self.gen_random_throw_dir();
        coin.clone().bind_mut().launch_coin(dir, str);
    }

    fn gen_random_throw_dir(&mut self) -> (Vector3, f32) {
        let mut rng = rand::thread_rng();
        let strength = rng.gen_range(5.0..10.0_f32);

        let throw_dir = self.base().get_global_transform().basis.col_c();

        let yaw = rng.gen_range(-30.0_f32..30.0).to_radians(); // left/right spread
        let pitch = rng.gen_range(-15.0_f32..15.0).to_radians(); // up/down spread

        let rot_yaw = Quaternion::from_axis_angle(Vector3::UP, yaw);
        let rot_pitch = Quaternion::from_axis_angle(Vector3::RIGHT, pitch);
        let rotation = rot_yaw * rot_pitch;

        return (rotation.normalized() * (-throw_dir), strength);
    }
}
