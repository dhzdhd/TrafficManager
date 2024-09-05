use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum SimState {
    #[default]
    Playing,
    Paused,
}

pub struct SimStatePlugin;

impl Plugin for SimStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
    }
}
