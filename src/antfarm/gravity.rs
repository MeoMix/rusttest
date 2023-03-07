use bevy::prelude::*;

use super::elements::{AffectedByGravity, Element, Elements2D, Position};

// TODO: Add support for loosening neighboring sand.
// TODO: Add support for crushing deep sand.
// TODO: Add support for sand falling left/right randomly.
pub fn sand_gravity_system(
    mut sand_query: Query<(&Element, &mut Position, &mut Transform), With<AffectedByGravity>>,
    mut non_sand_query: Query<
        (&Element, &mut Position, &mut Transform),
        Without<AffectedByGravity>,
    >,
    mut elements2d_query: Query<&mut Elements2D>,
) {
    let mut elements2d = elements2d_query.single_mut();

    // For each sand element, look beneath it in the 2D array and determine if the element beneath it is air.
    // For each sand element which is above air, swap it with the air beneath it.
    for (_, mut sand_position, mut sand_transform) in sand_query.iter_mut() {
        if let Some(element_below_sand) = elements2d.get(sand_position.x, sand_position.y + 1) {
            // If there is air below the sand then continue falling down.
            if let Ok((element, mut air_position, mut air_transform)) =
            non_sand_query.get_mut(*element_below_sand) && *element == Element::Air {
                // Swap elements in 2D vector to ensure they stay consistent with position and translation
                elements2d.swap(air_position.as_ref(), sand_position.as_ref());

                // TODO: It seems like a good idea to keep model/view concerns separate, but could drop position entirely and rely on translation.
                // Swap element positions
                (sand_position.y, air_position.y) = (air_position.y, sand_position.y);

                // TODO: I could swap the Vec references instead of updating y, but that seems like a bad idea.
                // Reflect the updated position visually
                sand_transform.translation.y = -(sand_position.y as f32);
                air_transform.translation.y = -(air_position.y as f32);
            } else {
                // Otherwise, likely at rest, but potential for tipping off a precarious ledge.
                // Look for a column of air two units tall to either side of the sand and consider going in one of those directions.
                // if let Some(element_left_sand) = elements2d.0.get(sand_position.y * world_state.width + sand_position.x - 1) {


                // }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::antfarm::{elements::*, *};
    use wasm_bindgen_test::wasm_bindgen_test;

    // Confirm that sand ontop of air falls downward.
    #[wasm_bindgen_test]
    fn did_drop_sand() {
        let mut app = App::new();

        let (width, height) = (1, 2);

        let mut elements_2d = Vec::with_capacity((width * height) as usize);

        // Setup test entities
        let sand_id = app
            .world
            .spawn((
                ElementBundle::create_sand(Vec3::ZERO),
                Position { x: 0, y: 0 },
                AffectedByGravity,
            ))
            .id();
        let air_id = app
            .world
            .spawn((
                ElementBundle::create_air(Vec3::new(0.0, -1.0, 0.0)),
                Position { x: 0, y: 1 },
            ))
            .id();

        elements_2d.push(sand_id);
        elements_2d.push(air_id);

        let elements_2d_id = app
            .world
            .spawn(Elements2D::new(width, height, elements_2d))
            .id();

        // Add gravity system
        app.add_system(sand_gravity_system);
        // Run systems
        app.update();

        let updated_elements_2d = app.world.get::<Elements2D>(elements_2d_id);
        assert_eq!(updated_elements_2d.unwrap().get(0, 0), Some(&air_id));
        assert_eq!(updated_elements_2d.unwrap().get(0, 1), Some(&sand_id));

        assert_eq!(app.world.get::<Position>(sand_id).unwrap().y, 1);
        assert_eq!(app.world.get::<Position>(air_id).unwrap().y, 0);

        assert_eq!(
            app.world.get::<Transform>(sand_id).unwrap().translation.y,
            -1.0
        );
        assert_eq!(
            app.world.get::<Transform>(air_id).unwrap().translation.y,
            0.0
        );
    }

    // Confirm that sand ontop of non-air stays put
    #[wasm_bindgen_test]
    fn did_not_drop_sand() {
        let mut app = App::new();

        let (width, height) = (1, 2);

        let mut elements_2d = Vec::with_capacity((width * height) as usize);

        // Setup test entities
        let sand_id = app
            .world
            .spawn((
                ElementBundle::create_sand(Vec3::ZERO),
                Position { x: 0, y: 0 },
                AffectedByGravity,
            ))
            .id();
        let dirt_id = app
            .world
            .spawn((
                ElementBundle::create_dirt(Vec3::new(0.0, -1.0, 0.0)),
                Position { x: 0, y: 1 },
            ))
            .id();

        elements_2d.push(sand_id);
        elements_2d.push(dirt_id);

        let elements_2d_id = app
            .world
            .spawn(Elements2D::new(width, height, elements_2d))
            .id();

        // Add gravity system
        app.add_system(sand_gravity_system);

        // Run systems
        app.update();

        let updated_elements_2d = app.world.get::<Elements2D>(elements_2d_id);
        assert_eq!(updated_elements_2d.unwrap().get(0, 0), Some(&sand_id));
        assert_eq!(updated_elements_2d.unwrap().get(0, 1), Some(&dirt_id));

        assert_eq!(app.world.get::<Position>(dirt_id).unwrap().y, 1);
        assert_eq!(app.world.get::<Position>(sand_id).unwrap().y, 0);

        assert_eq!(
            app.world.get::<Transform>(dirt_id).unwrap().translation.y,
            -1.0
        );
        assert_eq!(
            app.world.get::<Transform>(sand_id).unwrap().translation.y,
            0.0
        );
    }

    // Confirm that sand at the bottom of the world doesn't panic
    #[wasm_bindgen_test]
    fn did_respect_bounds() {
        let mut app = App::new();

        let (width, height) = (1, 1);

        let mut elements_2d = Vec::with_capacity((width * height) as usize);

        // Setup test entities
        let sand_id = app
            .world
            .spawn((
                ElementBundle::create_sand(Vec3::ZERO),
                Position { x: 0, y: 0 },
                AffectedByGravity,
            ))
            .id();

        elements_2d.push(sand_id);

        let elements_2d_id = app
            .world
            .spawn(Elements2D::new(width, height, elements_2d))
            .id();

        // Add gravity system
        app.add_system(sand_gravity_system);
        // Run systems
        app.update();

        let updated_elements_2d = app.world.get::<Elements2D>(elements_2d_id);
        assert_eq!(updated_elements_2d.unwrap().get(0, 0), Some(&sand_id));

        assert_eq!(app.world.get::<Position>(sand_id).unwrap().y, 0);

        assert_eq!(
            app.world.get::<Transform>(sand_id).unwrap().translation.y,
            0.0
        );
    }
}
