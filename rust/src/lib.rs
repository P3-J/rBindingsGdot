use godot::prelude::*;

mod arena;
mod bucky;
mod car;
mod coin;
mod npc_throw_spot;
mod player;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
