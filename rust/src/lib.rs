use godot::prelude::*;

mod grid;
mod player;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

struct test {
    a: grid::slot::Saturn,
}
