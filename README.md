# Dungeon Crawler

Welcome to the project! This is a top-down RPG built using open-source and indie game assets.

## World-map prototype

Run `cargo run` to open the campaign globe. The planet uses a procedurally
rendered pixel sphere: continents, coastlines and mission flags wrap around its
surface while the helicopter remains in view. No downloaded assets are required.

- WASD / arrow keys: fly around the globe; Shift: boost.
- Tab: select the next mission and fly there automatically.
- Space: toggle autopilot to the selected mission.
- Enter near a destination: open its placeholder briefing. Escape: return.

This is a navigation prototype; briefings do not launch playable levels yet.
Terrain and mission coordinates live in `src/world_map.rs`. The existing
character assets and components are retained for future mission gameplay.

`cargo test` checks spherical projection and longitude wrapping.
`cargo run -- --smoke-test` opens a window, captures three views under
`target/world-map*.png`, and exits automatically.

## 🎨 Asset Credits & Acknowledgments

This project utilizes amazing graphical assets created by the community. Full credit goes to the original authors:

### 1. Tileset
* **Creator:** George Bailey
* **Source:** [OpenGameArt.org](https://opengameart.org/users/george)

### 2. Character Sprites
* **Asset Pack:** HD 8-Directional Top-Down Character Pack 1
* **Creator:** smallScaleInt
* **Source:** [itch.io - smallScaleInt](https://smallscaleint.itch.io/hd-8-directional-top-down-character-pack-1)
