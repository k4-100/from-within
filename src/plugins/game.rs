use bevy::{
  prelude::*, 
  // input::keyboard::*
};
// use rand::prelude::*;

// mod super::super::utils;

pub use super::super::utils::{
  components,
  resources
};


// region: --- PlayerPlugin itself
pub struct GamePlugin;

impl Plugin for GamePlugin{
  fn build(&self, app: &mut App){
    app
        .add_startup_system(setup)
    ;
  }
}

// endregion: --- PlayerPlugin itself



// region: --- PlayerPlugin systems

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    println!("words");
}

// endregion: --- PlayerPlugin systems
