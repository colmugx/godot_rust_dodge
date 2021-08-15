use gdnative::prelude::*;

mod main_scene;
mod sprites;
mod hud;

use main_scene::Main;
use hud::Hud;
use sprites::{enemy::Enemy, player::Player};

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Enemy>();
    handle.add_class::<Hud>();
    handle.add_class::<Main>();
}

godot_init!(init);
