use super::point;
use uuid::Uuid;

use bevy::prelude::*;

#[derive(Bundle)]
pub struct AntSpriteBundle {
    sprite_bundle: SpriteBundle,
    // TODO: is it a code smell that these are pub?
    pub facing: Facing,
    pub angle: Angle,
    pub behavior: Behavior,
}

#[derive(Bundle)]
pub struct AntLabelBundle {
    text_bundle: Text2dBundle,
}

impl AntLabelBundle {
    pub fn new(label: String, asset_server: &Res<AssetServer>) -> Self {
        Self {
            text_bundle: Text2dBundle {
                transform: Transform {
                    translation: Vec3::new(-ANT_WIDTH / 4.0, -1.5, 100.0),
                    scale: Vec3::new(0.05, 0.05, 0.0),
                    ..default()
                },
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        color: Color::rgb(0.0, 0.0, 0.0),
                        font_size: 12.0,
                        ..default()
                    },
                ),
                ..default()
            },
        }
    }
}

// TODO: dynamically infer width/height of image rather than hardcoding.
// https://stackoverflow.com/questions/70657798/get-width-and-height-from-an-image-in-bevy
const ANT_IMAGE_WIDTH: f32 = 184.0;
const ANT_IMAGE_HEIGHT: f32 = 154.0;
// 1.2 is just a feel good number to make ants slightly larger than the elements they dig up
const ANT_WIDTH: f32 = 1.2;
const ANT_HEIGHT: f32 = 1.2;

impl AntSpriteBundle {
    pub fn new(
        color: Color,
        facing: Facing,
        angle: Angle,
        behavior: Behavior,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("images/ant.png"),
                transform: Transform {
                    scale: Vec3::new(
                        ANT_WIDTH / ANT_IMAGE_WIDTH,
                        ANT_HEIGHT / ANT_IMAGE_HEIGHT,
                        0.0,
                    ),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            },
            facing,
            angle,
            behavior,
        }
    }
}

// TODO: It feels weird to express this as a component, but maybe I just need time.
#[derive(Component, PartialEq)]
pub enum Behavior {
    Wandering,
    Carrying,
}

// TODO: It feels weird to express this as a component, but maybe I just need time.
#[derive(Component, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

// TODO: It feels weird to express this as a component, but maybe I just need time.
// TODO: it's awkward that these aren't numbers
#[derive(Component, PartialEq)]
pub enum Angle {
    Zero,
    Ninety,
    OneHundredEighty,
    TwoHundredSeventy,
}

pub struct Ant {
    id: Uuid,
    location: point::Point,
    behavior: Behavior,
    facing: Facing,
    angle: Angle,
    timer: i32,
    name: String,
    active: bool,
}

impl Ant {
    pub fn new(
        x: i32,
        y: i32,
        behavior: Behavior,
        facing: Facing,
        angle: Angle,
        name: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            location: point::Point { x, y },
            behavior,
            facing,
            angle,
            // timer: getTimer(behavior),
            timer: 6,
            name,
            active: true,
        }
    }
}

// const BehaviorTimingFactors = {
//   wandering: 4,
//   carrying: 5,
// }

// export const getTimer = (behavior: Behavior) => BehaviorTimingFactors[behavior] + Math.floor((Math.random() * 3)) - 1;

// fn getTimer(behavior: Behavior) {
//     6
// }
