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

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // let my_gltf = asset_server.load("objects/cube1.glb#Scene0");
    // commands.spawn(Camera2dBundle{
    //   transform: Transform{
    //     scale: Vec3{x: 5.0, y: 5.0, z: 1.0},
    //     ..default()
    //   },
    //   ..default()
    // });

   commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 3.0)
            // .looking_at( Vec3::new(0., 0., 0.), Vec3::new(0.,0.,-0.5) )
            .with_scale( Vec3{x: 5.0, y: 5.0, z: 1.0}),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
        material: materials.add(Color::rgb(0., 0., 100.).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    // commands.spawn(SceneBundle {
    //     scene: my_gltf,
    //     transform: Transform::from_xyz(0.0, 0.0, 1.5),
    //     ..Default::default()
    // });

}


// endregion: --- PlayerPlugin systems
