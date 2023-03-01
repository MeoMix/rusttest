use bevy::{prelude::*, sprite::Anchor};

// TODO: Should this be more like Element { type: Dirt }?
#[derive(Component)]
struct Dirt;

#[derive(Bundle)]
pub struct DirtBundle {
    sprite_bundle: SpriteBundle,
    dirt: Dirt,
}

impl DirtBundle {
    pub fn new(position: Vec3) -> Self {
        DirtBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::hex("836539").unwrap(),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            dirt: Dirt,
        }
    }
}
