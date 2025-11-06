# Rust3dAimTrainer

A Kovaak's inspired 3d FPS Aim Trainer built from scratch in Rust.

This is only a demo version.

## Upcoming Features:
- Wall Texturing
- Bean-Shaped and Humanoid Targets
- Randomly Moving Targets
- Phyics-Based Moving Targets
- Cone- and Octagon-Shaped Rooms
- Sound Effects
- Additional Scenarios
- On-Screen GUI

## Codebase Tree Structure

```
Rust3dAimTrainer\
├───.gitignore
├───Cargo.lock
├───Cargo.toml
├───README.md
└───src\
    ├───dev_config.toml
    ├───main.rs
    ├───todo.txt
    └───engine\
        ├───camera.rs
        ├───input.rs
        ├───mod.rs
        ├───runtime.rs
        ├───core\
        │   ├───config.rs
        │   ├───matrix.rs
        │   ├───mesh.rs
        │   ├───mod.rs
        │   ├───triangle.rs
        │   └───vector.rs
        ├───rendering\
        │   ├───clipping.rs
        │   ├───gui.rs
        │   ├───helpers.rs
        │   ├───mod.rs
        │   ├───projection.rs
        │   ├───rasterizer.rs
        │   └───window.rs
        └───scenario\
            ├───gun.rs
            ├───mod.rs
            ├───scenario.rs
            ├───scene.rs
            ├───stats.rs
            └───target.rs
    
```
