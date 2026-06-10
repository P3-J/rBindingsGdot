use godot::prelude::*;

mod car;
mod coin;
mod npc_throw_spot;
mod player;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
