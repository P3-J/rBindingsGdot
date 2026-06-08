use godot::classes::{Camera3D, IVehicleBody3D, Input, VehicleBody3D};
use godot::global::move_toward;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=VehicleBody3D)]
struct PlayerCar {
    base: Base<VehicleBody3D>,
    inputs: Gd<Input>,
    enabled: bool,
    #[export]
    car_cam: Option<Gd<Camera3D>>,
}

#[godot_api]
impl IVehicleBody3D for PlayerCar {
    fn init(base: Base<VehicleBody3D>) -> Self {
        Self {
            base,
            inputs: Input::singleton(),
            enabled: false,
            car_cam: None,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if !self.enabled {
            return;
        }

        if let Some(car_camera) = &self.car_cam {
            car_camera.is_current();
        }

        let current_steering = self.base().get_steering() as f64;

        let steering = move_toward(
            current_steering,
            (self.inputs.get_axis("right_rot", "left_rot") * 0.7) as f64,
            delta * 10.0,
        );

        self.base_mut().set_steering(steering as f32);

        let steering_angle = self.inputs.get_axis("backwards", "forward") * 200.0;
        self.base_mut().set_engine_force(steering_angle);
    }
}
