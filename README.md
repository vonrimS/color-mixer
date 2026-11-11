# Color-mixer

**A small Bevy project: clean ECS boundaries, States, and input/output separation.**

A 1D physics and color-mixing simulation built with Bevy 0.20 (Newton's Cradle style blocks with elastic momentum transfer, dynamic window boundary collision, and smooth impact feedback). The game itself is simple — the point is the architecture behind it.

---

## What This Project Proves

- **States work cleanly.** `Menu` and `Playing` are separate states, with `OnEnter`/`OnExit` handling setup and cleanup. No entities leak between states.
- **Input and state change are separate systems.** One system reads input, another reacts to it. They are not merged into a monolithic `update`.
- **Custom 1D Physics & Visual Polish.** Coordinate-sorting collision resolution, elastic momentum transfer, and procedural interpolation (fade-out impact flash) implemented idiomatically in Bevy ECS.
- **Minimal UI is built with Bevy's native UI.** No external UI crate, no custom renderer — just `Node`, `Text`, and native components.

This is the Level 1 signal for gamedev hiring: I understand how data, systems, and spatial/visual logic are separated in Bevy, not just how to copy a tutorial.

---

## The Game

- **Menu**: press SPACE (or click the button) to start.
- **Playing**: interactive blocks bounce off window edges and each other with elastic collisions, flashing and smoothly fading back from white on impact. Click anywhere to mix the background color.
- **Back to menu**: right-click (temporary, for testing state transitions).

The project is intentionally compact. The architecture and clean implementation are the deliverables.

---

## Architecture Notes

### Why States, not a boolean flag
`AppState` is a derived `States` enum with `Menu` and `Playing`. Using `OnEnter`/`OnExit` means setup and cleanup are tied to the state itself, not scattered across systems. When the state changes, the menu UI is despawned automatically — no manual `if state == Menu { despawn }` checks.

### Why input and action are separate systems
`menu_input` only reads input and calls `next_state.set(...)`. It does not spawn, despawn, or mutate game data. Game input and physics run in decoupled systems, meaning adding new features (like gamepad support or external forces) touches isolated modules rather than the whole loop.

### Why `MixColor` is a Resource
The current mixed color is global state, not tied to a specific entity. A `Resource` is the right fit: it can be read by the background renderer, UI, and event listeners without polluting entity hierarchies.

### 1D Physics & Procedural Polish
- **Coordinate Sorting**: Blocks are sorted along the X-axis per frame to efficiently handle neighbor-to-neighbor elastic collisions and position correction.
- **Fade-out Flash Effect**: Impacts trigger a transient `FlashTimer` component, utilizing `Timer::fraction()` and `Srgba` linear interpolation to smoothly fade blocks back to their original colors.

---

## Project Structure
```bash
color-mixer/
├── Cargo.toml
├── assets/
└── src/
    ├── main.rs         # Entry point: engine initialization & plugins
    ├── states.rs       # AppState enum
    ├── components.rs   # ECS components (BlockColor, Velocity, FlashTimer)
    ├── resources.rs    # Global resources (MixColor)
    ├── events.rs       # Custom game events (ColorMixedEvent)
    └── screens/
        ├── mod.rs      # Screen module re-exports
        ├── menu.rs     # Menu setup and input systems
        └── game.rs     # 1D physics loop, collisions, and fade-out animations

---

## How to Run

```bash
cargo run
```