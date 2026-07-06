mod vector;
mod physics_body;
mod physics_engine;

use std::env;
use std::fs::File;
use std::io::Write;
use chrono::Utc; // Need to add this dependency
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
    
    // Create physics engine with floor collision
    let mut engine = PhysicsEngine::new();
    engine.set_floor_level(0.0);  // Ground level at y=0
    engine.set_elasticity(0.4);   // 80% energy retention on bounce
    
    engine.add_body(body);
    
    // Run simulation for specified duration
    let data = run_simulation_with_data(&mut engine, duration_seconds);
    
    // Export to CSV
    export_to_csv(&data, duration_seconds);
}

fn run_simulation_with_data(engine: &mut PhysicsEngine, duration_seconds: f64) -> Vec<(f64, f64, f64, f64)> {
    let time_step = 0.1; // 100ms time steps
    let num_iterations = (duration_seconds / time_step) as usize;
    
    println!("Running simulation for {} seconds ({} iterations)", duration_seconds, num_iterations);
    
    let mut data = Vec::new();
    
    // Initial state
    if let Some(body) = engine.bodies.get(0) {
        data.push((0.0, body.position.x, body.position.y, body.position.z));
    }
    
    for i in 0..num_iterations {
        engine.update(time_step);
        
        // Record data every iteration for detailed output
        if let Some(body) = engine.bodies.get(0) {
            data.push(((i + 1) as f64 * time_step, body.position.x, body.position.y, body.position.z));
        }
        
        // Print status every 10 iterations
        if i % 10 == 0 {
            if let Some(body) = engine.bodies.get(0) {
                println!("Step {}: Position = ({:.2}, {:.2}, {:.2}), Velocity = ({:.2}, {:.2}, {:.2})", 
                         i, body.position.x, body.position.y, body.position.z,
                         body.velocity.x, body.velocity.y, body.velocity.z);
            }
        }
    }
    
    data
}

fn export_to_csv(data: &Vec<(f64, f64, f64, f64)>, duration: f64) {
    let now = Utc::now();
    let timestamp = now.format("%Y-%m-%d-%H-%M-%S").to_string();
    let filename = format!("out/sim-{}.csv", timestamp);
    
    let mut file = File::create(&filename).expect("Could not create CSV file");
    
    // Write header
    writeln!(file, "time,x,y,z").expect("Could not write to CSV file");
    
    // Write data
    for (time, x, y, z) in data {
        writeln!(file, "{},{:.6},{:.6},{:.6}", time, x, y, z).expect("Could not write to CSV file");
    }
    
    println!("\nData exported to {} with {} records", filename, data.len());
}
