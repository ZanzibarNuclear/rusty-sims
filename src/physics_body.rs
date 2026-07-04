use crate::vector::Vector3;

#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
    pub mass: f64,
    pub radius: f64,
}

impl PhysicsBody {
    pub fn new(position: Vector3, velocity: Vector3, mass: f64, radius: f64) -> Self {
        PhysicsBody {
            position,
            velocity,
            acceleration: Vector3::zero(),
            mass,
            radius,
        }
    }

    pub fn update_position(&mut self, dt: f64) {
        self.position = self.position + self.velocity * dt;
    }

    pub fn update_velocity(&mut self, dt: f64) {
        self.velocity = self.velocity + self.acceleration * dt;
    }

    pub fn apply_force(&mut self, force: Vector3) {
        // F = ma => a = F/m
        self.acceleration = self.acceleration + force * (1.0 / self.mass);
    }

    pub fn reset_acceleration(&mut self) {
        self.acceleration = Vector3::zero();
    }
}