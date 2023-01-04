use bevy::{
  prelude::*, 
  input::keyboard::*, time::FixedTimestep
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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.025))
                .with_system(camera_movement)
        )
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
        transform: Transform::from_xyz(0., 0., 4.0)
            .looking_at( Vec3::new(0.,0.,0.), Vec3::new(0.,1.0,0.))
            .with_scale( Vec3{x: 5.0, y: 5.0, z: 1.0}),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(10., 10., 10.) )),
        material: materials.add(Color::rgb(0., 50., 0.).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
    //     material: materials.add(Color::rgb(0., 0., 100.).into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    // commands.spawn(SceneBundle {
    //     scene: my_gltf,
    //     transform: Transform::from_xyz(0.0, 0.0, 1.5),
    //     ..Default::default()
    // });

}

fn camera_movement( 
    mut key_evr: EventReader<KeyboardInput>, 
    mut camera: Query<&mut Transform, With<Camera>>
){
    // println!("{:?}", camera.iter().last());
    //

    for ev in key_evr.iter(){
        match ev.key_code{
            Some(x) => {
                let mut cmr = &mut camera.single_mut();

                println!("{:?}", x);
                match x{
                    KeyCode::W => { cmr.translation.z += -1.; },
                    KeyCode::S => { cmr.translation.z +=  1.; },
                    KeyCode::A => { cmr.translation.x += -1.; },
                    KeyCode::D => { cmr.translation.x += 1.; },
                    KeyCode::Space => { cmr.translation.y += 1.; },
                    KeyCode::C => { cmr.translation.y += -1.; },
                    KeyCode::Up => { cmr.rotation.x += -0.5; },
                    KeyCode::Down => { cmr.rotation.x += 0.5; },
                    KeyCode::Left => { cmr.translation.y += -1.; },
                    KeyCode::Right => { cmr.translation.y += -1.; },
                    _ => {}
                }
                // camera.single_mut().translation = translation;
                println!("{:?}", cmr);
            }
            None => println!("none")
        }
    }
}
// endregion: --- PlayerPlugin systems
