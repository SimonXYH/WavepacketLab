# Schrödinger Equation Simulator (1D & 2D, Rust)

In this project, I built a a numerical solver that can evolve any properly initialized wavefunction in 2D or 1D under a user-defined potential using 
the Schrodinger equation. Several built-in potentials 
are manually defined, such as the step, ramp, and double slits potential, which is already enough for the user to 
do many experiment by themselves. 

This was a passion project and my original motivation was simply to see what would happen if I send some Gaussian wave through a double 
slit potential. In making the simulation, I couldn't help but making it a bit more general so I could handle arbitrary potentials. The main 
things this project allowed me to do are:
- to gain intuition on how wavefunction evolves by playing with wavefunction evolving under different potentials
- to explore different potential geometries (steps, ramps, double slits, etc.)
- to visualize quantum dynamics interactively
---

Let's see what a typical application would look like, here the potential landscape consists of a ramp potential and a double slit potential:

<img width="736" height="541" alt="image" src="https://github.com/user-attachments/assets/44bbdb91-24f4-49b7-a381-c95d4a6010e5" />

A 'ramp' potential is used to accelerate a gaussian wave packet toward the double slit potential.

<img width="743" height="538" alt="Screenshot 2025-12-28 213855" src="https://github.com/user-attachments/assets/96d75bac-f165-449f-bf60-8f39a994d389" />

## What’s implemented

- Time-dependent Schrödinger equation in 2D (1D is a special case)
- Finite-difference Laplacian
- RK4 integration
- Arbitrary, user-defined scalar potentials
- Complex-valued wavefunction (`Complex32`)
- Interactive SDL2 visualization
- Built-in example potentials:
  - Gaussian wave packets
  - Double slit
  - Step potential
  - Linear ramp
  - Harmonic oscillator (quadratic potential)

Main crates used:
- `ndarray` for grid-based storage
- `num-complex` for complex arithmetic
- `rayon` (optional parallelism)
- `sdl2` for visualization and interaction

---

## Code structure and abstractions

### 1. Math primitives and numeric types

At the lowest level are small, reusable math types:
- lightweight vector / tuple types (`Vec2`, `Vec3`, `Tup2`) for coordinates and rendering


---

### 2. Spatial grid and discretization

The simulation runs on a regular Cartesian grid.

- `create_grid(...)` builds a grid of physical `(x, y)` coordinates
- grid spacing (`dx`, `dy`) is computed explicitly
- the wavefunction `ψ(x, y)` is stored as an `Array2<Complex32>`


---

### 3. Differential operators (Laplacian)

Spatial derivatives are computed using finite differences.

- `d2_elementwise(...)` applies a 5-point stencil approximation of the Laplacian
- this corresponds directly to the kinetic energy term `∇²ψ`

---

### 4. Quantum state (`WaveVector2D`)

`WaveVector2D` represents the complete quantum state of the system.

It stores:
- the spatial grid
- the wavefunction values `ψ`
- the particle mass
- the probability density `|ψ|²`

Its responsibilities:
- compute the time derivative of the wavefunction
- apply the Schrödinger evolution operator
- update the wavefunction using RK4

This is the core physics abstraction of the solver.

---

### 5. Potentials
<img width="1091" height="571" alt="image" src="https://github.com/user-attachments/assets/37dd735d-c6ab-4d76-9610-f096e536ac17" />


Potentials are implemented as small, composable structs:
- `DoubleSlit`
- `StepPotential`
- `RampPotential`
- harmonic oscillator

Each potential:
- contains a `potential_value(x, y)` function
- returns a scalar potential at a point

Potentials are evaluated on the grid and summed, making it easy to construct complex
experimental setups.

---

### 6. Time integration

Time evolution is handled using **Runge–Kutta 4**

---

### 7. Visualization and interaction

The top layer handles visualization and user interaction:
- SDL2 window and event loop
- 3D-style rendering of `|ψ|²` with mesh 
- potential overlaid for reference
- keyboard and mouse controls for navigation

---

## Example scenarios

With the current setup, the simulator can reproduce:
- Gaussian wave packet propagation
- Reflection and transmission at step potentials
- Double-slit interference
- Acceleration under linear ramps
- Quantum harmonic oscillator
---

## Possible next steps
-QFT...




