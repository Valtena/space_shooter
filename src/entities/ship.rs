//! Manage the ship entity

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};

use super::png_mesh_and_material;

// The width and the height come from the png file
const SHIP_WIDTH:f32 = 105.0;
const SHIP_HEIGHT:f32 = 83.0;


use components::Ship;

/// Initialises the player's ship at the bottom centre of the screen
///
/// The function creates a ship sprite (mesh and material),
/// sets up the transform to scale the sprite and to position it
/// at the bottom (y=0) centre of the screen,
/// then finally bundles all the components into an entity.
pub fn initialise_ship(world: &mut World) -> Entity {
    let (mesh, material) = png_mesh_and_material(
        "PNG/playerShip4_purple.png",
        [SHIP_WIDTH, SHIP_HEIGHT], world);
    let width = {
        /*
        let dim = world.read_resource::<ScreenDimensions>();
        dim.width()
        */
        1024.
    };

    // Set the scale and position of our ship sprite:
    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(width/2.0 - SHIP_WIDTH/2.0, 0.0, 0.0);

    // Create a new entity by bundling the mesh, material, component and transforms together
    // then return the entity we created.
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Ship {
            velocity: 0.0, // ship starts out stationary
            width: SHIP_WIDTH,
            height: SHIP_HEIGHT,
            trigger_reset_timer: 0.0})
        .with(local_transform)
        .with(Transform::default())
        .build()
}
