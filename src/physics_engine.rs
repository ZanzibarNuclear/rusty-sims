use crate::vector::Vector3;
use crate::physics_body::PhysicsBody;

pub struct PhysicsEngine {
    pub bodies: Vec<PhysicsBody>,
    pub gravity: f64,
    pub floor_level: f64,
    pub elasticity: f64, // Coefficient of restitution (0.0 = no bounce, 1.0 = perfect bounce)
}

impl PhysicsEngine {
    pub fn new() -> PhysicsEngine {
        PhysicsEngine {
            bodies: Vec::new(),
            gravity: 9.81, // m/s^2
            floor_level: 0.0,
            elasticity: 0.8, // 80% energy retention on bounce
        }
    }

    pub fn add_body(&mut self, body: PhysicsBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, dt: f64) {
        for body in &mut self.bodies {
            // Apply gravity
            body.acceleration.y = -self.gravity;
            
            // Update velocity and position
            body.velocity.x += body.acceleration.x * dt;
            body.velocity.y += body.acceleration.y * dt;
            body.velocity.z += body.acceleration.z * dt;
            
            body.position.x += body.velocity.x * dt;
            body.position.y += body.velocity.y * dt;
            body.position.z += body.velocity.z * dt;
            
            // Handle floor collision
            if body.position.y - body.radius < self.floor_level {
                body.position.y = self.floor_level + body.radius;
                body.velocity.y = -body.velocity.y * self.elasticity;
                
                // If velocity is very small, stop bouncing
                if body.velocity.y.abs() < 0.1 {
                    body.velocity.y = 0.0;
                }
            }
        }
    }

    pub fn set_floor_level(&mut self, level: f64) {
        self.floor_level = level;
    }

    pub fn set_elasticity(&mut self, elasticity: f64) {
        self.elasticity = elasticity;
    }
}
