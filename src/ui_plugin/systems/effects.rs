#![allow(clippy::duplicate_mod)]
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use super::ui_helpers::ParticlesEffect;

use crate::components::EffectsCamera;

pub fn update_particles_effect(
    mut q_effect: Query<(&mut bevy_hanabi::EffectSpawner, &mut Transform), Without<Projection>>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut effects_camera: Query<(&mut Camera, &GlobalTransform), With<EffectsCamera>>,
) {
}

pub fn create_particles_effect(
    mut query: Query<(&Interaction, &Children), (Changed<Interaction>, With<ParticlesEffect>)>,
    mut text_style_query: Query<&mut Text, With<ParticlesEffect>>,
    mut commands: Commands,
    mut effects: ResMut<Assets<bevy_hanabi::EffectAsset>>,
    mut effects_camera: Query<&mut Camera, With<EffectsCamera>>,
    mut effects_query: Query<(&Name, Entity)>,
) {
}
