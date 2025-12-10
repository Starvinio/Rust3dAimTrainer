# Rust3dAimTrainer

A Kovaak's inspired 3d FPS Aim Trainer built from scratch in Rust.

![Image](https://github.com/user-attachments/assets/79be8949-a5c7-4b89-9031-e727f385a23d)

## Features
- FPS style camera movement
- 20+ Custom Scenarios
- Randomly Moving Targets
- Score Display After Scenario Run
- Custom Settings

## How to change settings/add customization
1. Compile Application
2. Go to `/target/release/config.toml`
3. Make desired changes
<img width="617" height="631" alt="Image" src="https://github.com/user-attachments/assets/5b0980ea-38f9-4de1-9ef3-296549d793d1" />

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
