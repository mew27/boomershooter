use bevy::prelude::{KeyCode, MouseButton};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RawInputContainer {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
}
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Copy, Clone)]
pub enum ButtonAction {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    Jump,
    Crouch,
    Fire,
    SecondaryFire,
}

fn get_default_input_map() -> HashMap<RawInputContainer, ButtonAction> {
    let mut configs = HashMap::<RawInputContainer, ButtonAction>::new();

    configs.insert(
        RawInputContainer::KeyCode(KeyCode::W),
        ButtonAction::MoveForward,
    );
    configs.insert(
        RawInputContainer::KeyCode(KeyCode::A),
        ButtonAction::MoveLeft,
    );
    configs.insert(
        RawInputContainer::KeyCode(KeyCode::S),
        ButtonAction::MoveBackward,
    );
    configs.insert(
        RawInputContainer::KeyCode(KeyCode::D),
        ButtonAction::MoveRight,
    );

    configs.insert(
        RawInputContainer::KeyCode(KeyCode::Space),
        ButtonAction::Jump,
    );
    configs.insert(
        RawInputContainer::KeyCode(KeyCode::LControl),
        ButtonAction::Crouch,
    );

    configs.insert(
        RawInputContainer::MouseButton(MouseButton::Left),
        ButtonAction::Fire,
    );
    configs.insert(
        RawInputContainer::MouseButton(MouseButton::Right),
        ButtonAction::SecondaryFire,
    );

    return configs;
}

pub fn load_input_map() -> HashMap<RawInputContainer, ButtonAction> {
    let path = Path::new("configs.json");

    match read_to_string("configs.json") {
        Ok(string) => serde_json::from_str(&string).unwrap(),
        Err(_) => get_default_input_map(),
    }
}
