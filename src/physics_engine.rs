use crate::physics_body::PhysicsBody;
use crate::vector::Vector3;

pub struct PhysicsEngine {
    pub bodies: Vec<PhysicsBody>,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: PhysicsBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, dt: f64) {
        // Reset accelerations
        for body in &mut self.bodies {
            body.reset_acceleration();
        }

        // Apply gravity to all bodies
        let gravity = Vector3::new(0.0, -9.81, 0.0);
        for body in &mut self.bodies {
            body.apply_force(gravity * body.mass);
        }

        // Update positions and velocities
        for body in &mut self.bodies {
            body.update_velocity(dt);
            body.update_position(dt);
        }
    }
}