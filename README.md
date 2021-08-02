# Seyral
A text-based game where you duel various enemies. Written in rust.
## Technologies
Uses TUI with a Crossterm backend for rendering the UI and listening for user input to
(hopefully) be cross platform. Serde for serialization/deserialization of data models,
and thiserror for helping with deriving errors into a more convenient wrapper.
### Crates:
- [Chrono](https://github.com/chronotope/chrono)
- [Crossterm](https://github.com/crossterm-rs)
- [Serde](https://github.com/serde-rs/serde)
- [thiserror](https://github.com/dtolnay/thiserror)
- [TUI](https://github.com/fdehau/tui-rs)
