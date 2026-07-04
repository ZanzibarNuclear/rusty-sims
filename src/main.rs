mod vector;
mod physics_body;
mod physics_engine;

use std::env;
use vector::Vector3;
use physics_body::PhysicsBody;
use physics_engine::PhysicsEngine;

fn main() {
    println!("Physics Simulation Platform - Run Loop Test");
    
    // Get duration from command line argument or default to 5 seconds
    let args: Vec<String> = env::args().collect();
    let duration_seconds: f64 = if args.len() > 1 {
        args[1].parse().expect("Please provide a valid number for duration")
    } else {
        5.0
    };
    
    // Create a simple test body
    let position = Vector3::new(0.0, 10.0, 0.0);
    let velocity = Vector3::new(5.0, 0.0, 0.0);
    let body = PhysicsBody::new(position, velocity, 1.0, 1.0);
    
    println!("Created body: {:?}", body);
    
    // Create a physics engine and add our body
    let mut engine = PhysicsEngine::new();
    engine.add_body(body);
    
    // Run simulation for specified duration
    run_simulation(&mut engine, duration_seconds);
}

fn run_simulation(engine: &mut PhysicsEngine, duration_seconds: f64) {
    let time_step = 0.1; // 100ms time steps
    let num_iterations = (duration_seconds / time_step) as usize;
    
    println!("Running simulation for {} seconds ({} iterations)", duration_seconds, num_iterations);
    
    for i in 0..num_iterations {
        engine.update(time_step);
        
        // Print status every 10 iterations
        if i % 10 == 0 {
            let body = &engine.bodies[0];
            println!("Step {}: Position = ({:.2}, {:.2}, {:.2}), Velocity = ({:.2}, {:.2}, {:.2})", 
                     i, body.position.x, body.position.y, body.position.z,
                     body.velocity.x, body.velocity.y, body.velocity.z);
        }
    }
    
    // Print final state
    let final_body = &engine.bodies[0];
    println!("Final state after {} seconds:", duration_seconds);
    println!("  Position: ({:.2}, {:.2}, {:.2})", 
             final_body.position.x, final_body.position.y, final_body.position.z);
    println!("  Velocity: ({:.2}, {:.2}, {:.2})", 
             final_body.velocity.x, final_body.velocity.y, final_body.velocity.z);
}
