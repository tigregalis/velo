use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::CosmicFont;
use bevy_prototype_lyon::prelude::Path;

use crate::{
    canvas::shadows::CustomShadowMaterial,
    resources::{AppState, FontSystemState},
    themes::Theme,
    utils::ReflectableUuid,
};

use super::{
    ui_helpers::{spawn_sprite_node, VeloShape},
    AddRect, NodeMeta, UiState,
};

pub fn create_new_node(
    mut commands: Commands,
    mut events: EventReader<AddRect<(String, Color)>>,
    mut ui_state: ResMut<UiState>,
    app_state: Res<AppState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
    mut z_index_local: Local<f32>,
    mut materials: ResMut<Assets<CustomShadowMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    existing_node_query: Query<(&Path, &GlobalTransform), With<VeloShape>>,
) {
    if events.is_empty() {
        return;
    }
    let node_rects = existing_node_query
        .iter()
        .filter_map(|(paths, transform)| {
            let extents = paths.0.iter().fold(
                (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN)),
                |(mut min, mut max), p| {
                    let from = p.from();
                    let to = p.to();
                    min.x = from.x.min(min.x);
                    min.x = to.x.min(min.x);

                    max.x = from.x.max(max.x);
                    max.x = to.x.max(max.x);

                    (min, max)
                },
            );
            if extents == (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN)) {
                None
            } else {
                dbg!(transform.translation());
                Some(Rect {
                    min: transform.translation().truncate() + extents.0,
                    max: transform.translation().truncate() + extents.1,
                })
            }
        })
        .collect::<Vec<_>>();
    dbg!(&node_rects);

    const GAP: f32 = 10.0;

    let window = windows.single_mut();
    for event in events.iter() {
        let new_position = Vec2::new(event.node.x, event.node.y);
        let new_quadrant = Vec2::new(event.node.width, event.node.height);
        dbg!(new_position);
        let mut new_rect = Rect {
            min: new_position - new_quadrant,
            max: new_position + new_quadrant,
        };

        for rect in node_rects.iter() {
            // found an intersection
            if !new_rect.intersect(*rect).is_empty() {
                // shift right only, try another
                new_rect = Rect {
                    min: rect.max + Vec2::new(GAP + new_rect.width(), 0.0),
                    max: rect.max + Vec2::new(GAP + new_rect.height(), 0.0),
                };
            }
            // // did not find an intersection, OK so far
            // if new_rect.intersect(*rect).is_empty() {
            //     continue;
            // }
        }
        dbg!(new_rect);

        let current_document = app_state.current_document.unwrap();
        let tab = app_state
            .docs
            .get(&current_document)
            .unwrap()
            .tabs
            .iter()
            .find(|x| x.is_active)
            .unwrap();
        *z_index_local += 0.01 % f32::MAX;
        *ui_state = UiState::default();
        ui_state.entity_to_edit = Some(ReflectableUuid(event.node.id));
        let _ = spawn_sprite_node(
            &mut commands,
            &mut materials,
            &mut meshes,
            &theme,
            &mut cosmic_fonts,
            font_system_state.0.clone().unwrap(),
            window.scale_factor() as f32,
            NodeMeta {
                id: ReflectableUuid(event.node.id),
                size: (event.node.width, event.node.height),
                node_type: event.node.node_type.clone(),
                image: event.image.clone(),
                text: event.node.text.text.clone(),
                pair_bg_color: event.node.bg_color.clone(),
                position: (
                    new_rect.center().x,
                    new_rect.center().y,
                    tab.z_index + *z_index_local,
                ),
                text_pos: event.node.text.pos.clone(),
                is_active: true,
            },
        );
    }
}
