use godot::prelude::*;

use crate::bucky::Bucky;

pub trait BuckyAnimationPlayer {
    fn play_animation(&mut self, anim_name: &str);
}

impl BuckyAnimationPlayer for Bucky {
    fn play_animation(&mut self, anim_name: &str) {
        let Some(anim_player) = &mut self.bucky_anim_player else {
            godot_error!("no anim player linked to bucky");
            return;
        };

        match anim_name {
            "stop" => {
                anim_player.stop();
            }
            "reset_soft" => {
                if anim_player.is_playing() {
                    anim_player.set_current_animation(&StringName::from("RESET"));
                }
            }
            "hang_ledge" => {
                let c_anim = anim_player.get_current_animation();
                if c_anim == anim_name {
                    return;
                }
                anim_player.set_current_animation(&StringName::from(anim_name));
                anim_player.play();
            }
            _ => {
                anim_player.set_current_animation(&StringName::from(anim_name));
                anim_player.play();
            }
        };
    }
}
