use godot::prelude::*;

mod car;
mod coin;
mod player;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
