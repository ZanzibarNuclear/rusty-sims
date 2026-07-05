# Rusty Sims - Physics Simulation Platform

A simple 3D physics simulation platform built in Rust with support for:
- Gravity simulation
- Floor collision detection and bouncing
- Position tracking and CSV data export
- Configurable time steps and simulation duration

## Features

- **3D Vector Mathematics**: Custom Vector3 implementation for position, velocity, and acceleration
- **Physics Engine**: Basic physics simulation with gravity and collision response
- **Data Export**: Automatically exports simulation data to CSV files with timestamps
- **Modular Design**: Clean separation of concerns with dedicated modules for vector math and physics

## Usage

Run the simulation with a specified duration in seconds:

```bash
cargo run <duration_seconds>
```

Example:
```bash
cargo run 5
```

This will create a CSV file with timestamped position data of the simulated object.

## Simulation Details

- **Gravity**: Constant downward acceleration (9.8 m/s²)
- **Floor Collision**: Objects bounce when they hit y=0 with energy loss (elasticity = 0.4)
- **Time Step**: 0.1 seconds per simulation step
- **Object Properties**: 
  - Mass: 1.0 kg
  - Radius: 1.0 meters
  - Initial position: (0, 10, 0) meters
  - Initial velocity: (5, 0, 0) m/s

## Files Created

The simulation creates CSV files with the following format:
```
time,x,y,z
0.0,0.000000,10.000000,0.000000
0.1,0.500000,9.901900,0.000000
...
```

## Project Structure

- `src/main.rs` - Main simulation loop and application entry point
- `src/vector3.rs` - 3D vector mathematics implementation
- `src/physics_engine.rs` - Physics simulation logic including gravity and collision detection

## Requirements

- Rust 1.56 or later
- Cargo (Rust's package manager)

## License

MIT License
