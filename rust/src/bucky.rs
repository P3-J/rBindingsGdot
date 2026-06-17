use godot::classes::input::MouseMode;
use godot::classes::{
    AnimationPlayer, Camera3D, CharacterBody3D, GpuParticles3D, ICharacterBody3D, Input,
    InputEvent, InputEventMouseMotion, PhysicsRayQueryParameters3D,
};
use godot::global::deg_to_rad;
use godot::prelude::*;

mod bucky_animation_manager;
use bucky_animation_manager::BuckyAnimationPlayer;

const SPEED: f32 = 14.0;
const ACCELERATION: f32 = 8.0;
const FRICTION: f32 = 10.0;

const JUMP_VELOCITY: f32 = 20.0;
const DOUBLE_JUMP_VELOCITY: f32 = 20.0;
const FALL_GRAVITY: f32 = 45.0;
const JUMP_GRAVITY: f32 = 50.0;

const WALL_HOP_VELOCITY: f32 = 20.0;
const WALL_HOP_NORMAL_FORCE: f32 = 25.0;
const WALL_SLIDE_GRAVITY: f32 = 2.0;
const WALL_SLIDE_SPEED_MAX: f32 = 2.0;

const TURN_SPEED: f32 = 12.0;
const WALL_RAY_LENGTH: f32 = 1.5;
const WALL_RAY_HEIGHT: f32 = 0.8;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Bucky {
    base: Base<CharacterBody3D>,
    movement_dir: Vector3,
    jumps_remaining: i32,
    is_wall_sliding: bool,
    wall_normal: Vector3,
    has_wall_jumped: bool,

    #[export]
    camera_base: Option<Gd<Camera3D>>,
    #[export]
    char_body_base: Option<Gd<Node3D>>,
    #[export]
    bucky_anim_player: Option<Gd<AnimationPlayer>>,
    #[export]
    bucky_jump_particle: Option<Gd<GpuParticles3D>>,
    #[export]
    bucky_camera_base: Option<Gd<Node3D>>,
}

#[godot_api]
impl ICharacterBody3D for Bucky {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            movement_dir: Vector3::ZERO,
            jumps_remaining: 2,
            is_wall_sliding: false,
            wall_normal: Vector3::ZERO,
            has_wall_jumped: false,
            camera_base: None,
            char_body_base: None,
            bucky_anim_player: None,
            bucky_jump_particle: None,
            bucky_camera_base: None,
        }
    }

    fn process(&mut self, delta: f64) {
        self.handle_input();
        self.handle_movement(delta as f32);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_camera_input(&event);
    }

    fn ready(&mut self) {
        let mut is = Input::singleton();
        is.set_mouse_mode(MouseMode::CAPTURED);
    }
}

impl Bucky {
    fn handle_input(&mut self) {
        let input = Input::singleton();
        let mut dir = Vector3::ZERO;

        let Some(cam) = &self.camera_base else { return };
        let basis = cam.get_global_transform().basis;
        let forward = -basis.col_c();
        let right = basis.col_a();

        if input.is_action_pressed("forward") {
            dir += forward;
        }
        if input.is_action_pressed("backwards") {
            dir -= forward;
        }
        if input.is_action_pressed("left_rot") {
            dir -= right;
        }
        if input.is_action_pressed("right_rot") {
            dir += right;
        }

        dir.y = 0.0;
        if dir.length() > 0.0 {
            dir = dir.normalized();
        }
        self.movement_dir = dir;
    }

    fn check_wall_contact(&self) -> Option<Vector3> {
        let Some(char_body) = &self.char_body_base else {
            return None;
        };

        let char_pos = self.base().get_global_position() + Vector3::new(0.0, WALL_RAY_HEIGHT, 0.0);
        let facing = -char_body.get_global_transform().basis.col_c();

        let world = self.base().get_world_3d()?;
        let mut space = world.get_direct_space_state()?;

        let mut params = PhysicsRayQueryParameters3D::new_gd();
        params.set_from(char_pos);
        params.set_to(char_pos + facing * WALL_RAY_LENGTH);
        params.set_exclude(&array![self.base().get_rid()]);

        let result = space.intersect_ray(&params);
        if result.is_empty() {
            return None;
        }

        let normal: Vector3 = result.get("normal")?.to();
        Some(normal)
    }

    fn handle_movement(&mut self, delta: f32) {
        let on_floor = self.base().is_on_floor();
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        if on_floor {
            self.jumps_remaining = 2;
            self.is_wall_sliding = false;
            self.has_wall_jumped = false;
        }

        self.is_wall_sliding = false;
        if !on_floor && velocity.y < 0.0 && !self.has_wall_jumped {
            if let Some(normal) = self.check_wall_contact() {
                self.wall_normal = normal;
                self.is_wall_sliding = true;
            }
        }

        if !on_floor {
            let gravity = if self.is_wall_sliding {
                self.play_animation("hang_ledge");
                WALL_SLIDE_GRAVITY
            } else if velocity.y > 0.0 {
                JUMP_GRAVITY
            } else {
                self.play_animation("drop");
                FALL_GRAVITY
            };
            velocity.y -= gravity * delta;

            if self.is_wall_sliding && velocity.y < -WALL_SLIDE_SPEED_MAX {
                velocity.y = -WALL_SLIDE_SPEED_MAX;
            }
        } else {
            if self.movement_dir.length() > 0.0 {
                self.play_animation("walk");
            } else {
                self.play_animation("reset_soft");
            }
        }

        if input.is_action_just_pressed("jump") {
            self.handle_jump_externals();
            if self.is_wall_sliding {
                velocity.x = self.wall_normal.x * WALL_HOP_NORMAL_FORCE;
                velocity.z = self.wall_normal.z * WALL_HOP_NORMAL_FORCE;
                velocity.y = WALL_HOP_VELOCITY;
                self.is_wall_sliding = false;
                self.has_wall_jumped = true;
            } else if on_floor {
                velocity.y = JUMP_VELOCITY;
                self.jumps_remaining -= 1;
            } else if self.jumps_remaining > 0 {
                velocity.y = DOUBLE_JUMP_VELOCITY;
                self.jumps_remaining -= 1;
            }
        }

        if self.movement_dir.length() > 0.0 {
            velocity.x = lerp(
                velocity.x,
                self.movement_dir.x * SPEED,
                ACCELERATION * delta,
            );
            velocity.z = lerp(
                velocity.z,
                self.movement_dir.z * SPEED,
                ACCELERATION * delta,
            );
        } else {
            velocity.x = lerp(velocity.x, 0.0, FRICTION * delta);
            velocity.z = lerp(velocity.z, 0.0, FRICTION * delta);
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        self.smooth_turn_body(delta);
    }

    fn smooth_turn_body(&mut self, delta: f32) {
        let Some(char_body) = &mut self.char_body_base else {
            return;
        };
        if self.movement_dir == Vector3::ZERO {
            return;
        }

        let current_basis = char_body.get_global_transform().basis;
        let forward = -self.movement_dir.normalized();
        let right = Vector3::UP.cross(forward).normalized();
        let real_up = forward.cross(right).normalized();
        let target_basis = Basis::from_cols(right, real_up, forward);

        let t = (TURN_SPEED * delta).min(1.0);
        let new_basis = current_basis.slerp(&target_basis, t);

        let mut transform = char_body.get_global_transform();
        transform.basis = new_basis;
        char_body.set_global_transform(transform);
    }

    fn handle_jump_externals(&mut self) {
        self.play_animation("jump");
        self.play_particle("jump");
    }

    fn handle_camera_input(&mut self, event: &Gd<InputEvent>) {
        let event_copy = event.clone();
        let Ok(mouse_event) = event_copy.try_cast::<InputEventMouseMotion>() else {
            return;
        };
        let delta = mouse_event.get_relative();

        if let Some(pivot) = &mut self.bucky_camera_base {
            let mut pivot_rot = pivot.get_rotation();
            pivot_rot.y -= delta.x * 0.005;
            pivot.set_rotation(pivot_rot);
        }

        if let Some(cam) = &mut self.camera_base {
            let mut cam_rot = cam.get_rotation();
            cam_rot.x -= delta.y * 0.005;
            cam_rot.x = cam_rot
                .x
                .clamp(deg_to_rad(-30.0) as f32, deg_to_rad(30.0) as f32);
            cam.set_rotation(cam_rot);
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
