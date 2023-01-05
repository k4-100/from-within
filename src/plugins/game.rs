use bevy::{
    prelude::*, 
    input::keyboard::*, time::FixedTimestep,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
// use rand::prelude::*;

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
        .insert_resource(resources::DebugInfo{ camera_transform: String::new(), fps: 0. })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.025))
                .with_system(camera_movement)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.025))
                .with_system(fps_refresh)
        )
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
    ;
  }
}

// endregion: --- PlayerPlugin itself



// region: --- PlayerPlugin systems

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    debug_info: Res<resources::DebugInfo>
){

   commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 10.0)
            .looking_at( Vec3::new(0.,0.,0.), Vec3::new(0.,1.0,0.))
            .with_scale( Vec3{x: 5.0, y: 5.0, z: 1.0}),
        ..default()
    });

    commands.spawn( (
        components::DebugText,
        TextBundle::from_section(
            debug_info.get_formatted_debug_info(),
            TextStyle {
                font: asset_server.load("fonts/Ubuntu-M.ttf"),
                font_size: 40.0,
                color: Color::rgba(0.4, 0.7, 0.2, 1.),
            }
        )
    ) );

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(10., 10., 10.) )),
    //     material: materials.add(Color::rgb(0., 50., 0.).into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
    
    
// commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
    //     material: materials.add(Color::rgb(0., 0., 100.).into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    let my_gltf = asset_server.load("objects/cube1.glb#Scene0");
    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 0.0, 1.5),
        ..Default::default()
    });

}

fn camera_movement( 
    mut key_evr: EventReader<KeyboardInput>, 
    mut camera: Query<&mut Transform, With<Camera>>,
    mut debug_text: Query<&mut Text, With<components::DebugText>>,
    mut debug_info_res: ResMut<resources::DebugInfo>
){
    for ev in key_evr.iter(){
        match ev.key_code{
            Some(x) => {
                let cmr = &mut camera.single_mut();
                
                println!("{:?}", x);
                match x{
                    KeyCode::W => { cmr.translation.z += -1.; },
                    KeyCode::S => { cmr.translation.z +=  1.; },
                    KeyCode::A => { cmr.translation.x += -1.; },
                    KeyCode::D => { cmr.translation.x += 1.; },
                    KeyCode::Space => { cmr.translation.y += 1.; },
                    KeyCode::C => { cmr.translation.y += -1.; },
                    KeyCode::Down => {
                        let angle =  (-10.0f32).to_radians();
                        cmr.rotate_x(angle);
                    },
                    KeyCode::Up => { 
                        let angle =  (10.0f32).to_radians();
                        cmr.rotate_x(angle);
                    },
                    KeyCode::Left => { 
                        let angle =  (10.0f32).to_radians();
                        cmr.rotate_y(angle);
                    },
                    KeyCode::Right => {
                        let angle =  (-10.0f32).to_radians();
                        cmr.rotate_y(angle);
                    },
                    _ => {}
                }

                /////////////////////////////////////////////////
                // Display debug camera info for camera
                /////////////////////////////////////////////////
                let debug_info = format!("{:?}", cmr);
                let debug_info_vec = debug_info
                    .split(|c: char| c == '{' || c == '}' );
                let debug_info_vec_replaced: Vec<String> = debug_info_vec
                    .into_iter().map( |item: &str| item.replace("),",")\n") ).collect();
                let debug_text_value = &mut debug_text.single_mut().sections[0].value;
                debug_info_res.camera_transform = format!("INFO: \n{}",debug_info_vec_replaced[1]);
                *debug_text_value = debug_info_res.get_formatted_debug_info().clone();
            },
            None => {}
        }
    }
}

fn fps_refresh(
    mut debug_text: Query<&mut Text, With<components::DebugText>>,
    diagnostics_res: Res<Diagnostics>,
    mut debug_info_res: ResMut<resources::DebugInfo>
){
    
    if let Some(fps) = diagnostics_res.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw) = fps.value() {
            debug_info_res.fps = (raw * 100.).round() / 100.0;
            let debug_text_value = &mut debug_text.single_mut().sections[0].value;
            *debug_text_value = debug_info_res.get_formatted_debug_info();
        }
    }
}

// endregion: --- PlayerPlugin systems
