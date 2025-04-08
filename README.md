Particle Simulation of an explosive

This is the first running version of a 3D particle simulation project written in Rust. It's my first attempt at building a simulation, and while it doesn't yet include physics analysis, the foundation for particle interactions has been implemented. The simulation is still a work in progress and focuses on the initial setup and particle movement.
Current Features

    Particle Initialization: Particles are created and initialized in a 3D space.

    Initial Velocity (t_0): Particles are initialized with an initial velocity (t_0), allowing them to move in the simulation.

    Interactions: Basic interactions between particles are set up.

    Energy Release: Energy release mechanisms have been implemented, though they are still in the early stages.

    Simulation Time Steps: The simulation currently runs for 100 time steps.

Known Limitations

    No Physics Analysis: There is no in-depth physics analysis yet; the focus is on the basic framework and interactions.

    Fixed Time Steps: The simulation is currently hard-coded to run for 100 time steps and doesn't yet handle longer or variable durations.

Installation

To run the simulation, you'll need to have Rust installed on your system. If you haven't installed it yet, you can get it from rust-lang.org.

Clone the repository:

git clone https://github.com/Florian-Elsen/Particle-Simulation.git

Navigate to the project directory:

cd Particle-Simulation

Build and run the project:

cargo run

Future Plans

    Implement detailed physics modeling for particle interactions.

    Expand the simulation to run for more than 100 time steps.

    Add the ability for real-time analysis of the simulation.

Contributing

Since this is an ongoing project, contributions are welcome! Feel free to fork the repository, submit pull requests, or open issues if you find bugs or have suggestions for improvements.
License

This project is licensed under the MIT License - see the LICENSE file for details.
