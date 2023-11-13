use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct SlideTimer(pub Timer);
