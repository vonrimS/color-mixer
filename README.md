# Color-mixer

**A small Bevy project: clean ECS boundaries, States, and input/output separation.**

A color-mixing game built with Bevy 0.20. Click colored blocks to blend the background color toward white. The game itself is simple — the point is the architecture behind it.

---

## What This Project Proves

- **States work cleanly.** `Menu` and `Playing` are separate states, with `OnEnter`/`OnExit` handling setup and cleanup. No entities leak between states.
- **Input and state change are separate systems.** One system reads input, another reacts to it. They are not merged into a monolithic `update`.
- **Minimal UI is built with Bevy's native UI.** No external UI crate, no custom renderer — just `Node`, `Text`, and `Button`.

This is the Level 1 signal for gamedev hiring: I understand how data and systems are separated in Bevy, not just how to copy a tutorial.

---

## The Game

- **Menu**: press SPACE (or click the button) to start.
- **Playing**: click colored blocks to mix the background color toward white.
- **Back to menu**: right-click (temporary, for testing state transitions).

The game is intentionally small. The architecture is the deliverable.

---

## Architecture Notes

### Why States, not a boolean flag

`AppState` is a derived `States` enum with `Menu` and `Playing`. Using `OnEnter`/`OnExit` means setup and cleanup are tied to the state itself, not scattered across systems. When the state changes, the menu UI is despawned automatically — no manual `if state == Menu { despawn }` checks.

### Why input and action are separate systems

`menu_input` only reads `ButtonInput<KeyCode>` and calls `next_state.set(...)`. It does not spawn, despawn, or mutate game data. `game_input` only handles gameplay input. This separation means:

- Adding a new input (e.g. gamepad) touches one system, not the whole game loop.
- Testing input logic does not require a full game world.

### Why `MixColor` is a Resource

The current mixed color is global state, not tied to a specific entity. A `Resource` is the right fit: it can be read by the background renderer, the UI, and future systems without passing it through entity hierarchies.

---

## Project Structure
```bash
color-mixer/
├── Cargo.toml
├── assets/
└── src/
    ├── main.rs         # Entry point: engine initialization & plugins
    ├── states.rs       # AppState enum
    ├── components.rs   # ECS components (BlockColor, MenuTag)
    ├── resources.rs    # Global resources (MixColor)
    └── screens/
        ├── mod.rs      # Screen module re-exports
        ├── menu.rs     # Menu setup and input systems
        └── game.rs     # Game setup and block click logic
```


Single-file project for Level 1. Later levels split into `screens/`, `game/`, and `ui/` modules.

---

## How to Run

```bash
cargo run
```