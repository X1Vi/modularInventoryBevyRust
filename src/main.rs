use bevy::{color::palettes::basic::*, prelude::*, winit::WinitSettings};

pub mod buttons {
    pub mod CustomButton;
}

use buttons::CustomButton::spawn_custom_button;
use buttons::CustomButton::update_button_information;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(Startup, spawn_custom_button)
        .add_systems(Update, update_button_information)
        .insert_resource(WinitSettings::desktop_app())
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
