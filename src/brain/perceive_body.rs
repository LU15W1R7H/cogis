use crate::{
    brain::*,
    corgi::{Corgi, Physique},
    universe::tile::{Tile, TileEntities},
};
use amethyst::{
    core::Transform,
    ecs::{prelude::*, System, SystemData},
    renderer::{palette::Hsl, resources::Tint},
};
use std::{collections::HashMap, sync::Mutex};

// This is a temporary system which takes care of handling values
// inside `Perception` which are not taken care of anywhere else.

#[derive(Default)]
pub struct PerceiveBodySystem;

impl<'s> System<'s> for PerceiveBodySystem {
    type SystemData = (
        WriteStorage<'s, BodyPerception>,
        ReadStorage<'s, Corgi>,
        WriteStorage<'s, Physique>,
    );

    fn run(&mut self, (mut perceptions, corgis, physiques): Self::SystemData) {
        for (perception, corgi, physique) in (&mut perceptions, &corgis, &physiques).join() {
            *perception = BodyPerception {
                energy: IoF32(corgi.energy),
                mass: IoF32(physique.mass),
            };
        }
    }
}
