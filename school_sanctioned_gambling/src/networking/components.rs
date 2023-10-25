use bevy::prelude::*;

#[derive(Component)]
pub struct NBundle;

#[derive(Component)]
pub struct JoinServerButton;

#[derive(Component)]
pub struct ExitServerButton;

#[derive(Component, Default)]
pub struct TextBox {
    pub text_style: TextStyle,
}

#[derive(Component)]
pub struct TextBoxTag;
