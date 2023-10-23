use bevy::prelude::*;

#[derive(Component)]
pub struct Options;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct NBundle;

#[derive(Component, Default, Debug)]
pub struct TextBox {
    pub active: bool,
    pub id: u32,
    pub text_style: TextStyle,
}

#[derive(Component)]
pub struct TextBoxTag {
    pub id: u32,
}