mod vector;
mod physics_body;
mod physics_engine;

use vector::Vector3;
use physics_body::PhysicsBody;
use physics_engine::PhysicsEngine;

fn main() {
    println!("Physics Simulation Platform - Basic Test");
    
    // Create a simple test body
    let position = Vector3::new(0.0, 10.0, 0.0);
    let velocity = Vector3::new(5.0, 0.0, 0.0);
    let body = PhysicsBody::new(position, velocity, 1.0, 1.0);
    
    println!("Created body: {:?}", body);
    
    // Create a physics engine and add our body
    let mut engine = PhysicsEngine::new();
    engine.add_body(body);
    
    // Run a simple simulation step
    engine.update(0.1);
    
    println!("After update: {:?}", engine.bodies[0]);
}
