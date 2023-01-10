use bevy::{
    prelude::*, 
    input::{keyboard::*, mouse::MouseMotion}, time::FixedTimestep,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin },

};
// use rand::prelude::*;

use std::fs;

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



    let my_map_string: String = fs::read_to_string("assets/map/1.txt")
        .expect("Should have been able to read the file");
    
    assert!(my_map_string.contains('P') );

    let my_map_string_vec: Vec<&str> = my_map_string.split_whitespace().collect();
    // let mut my_map_value_vec: Vec<(char,f64,f64)> = Vec::new();
    for (a, a_item) in my_map_string_vec.iter().enumerate(){
        for (b, b_item) in a_item.chars().enumerate(){
            // my_map_value_vec.push( (y_item, 100. * (y as f64), 100. * (x as f64) ) );
            match b_item{
                '#' =>{
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
                        material: materials.add(Color::rgb(0., 0., 100.).into()),
                        transform: Transform::from_xyz( 3.0 * (b as f32)  , 0.0,  3.0 * (a as f32)),
                        ..default()
                    });
                },
                'P' =>{
                    commands.spawn(Camera3dBundle {
                        transform: Transform::from_xyz(3.0 * b as f32, 0., 3.0 * a as f32)
                            .looking_at( Vec3::new(0.,0.,0.), Vec3::new(0.0,1.0,0.))
                            .with_scale( Vec3{x: 5.0, y: 5.0, z: 1.0}),
                        ..default()
                    });
                }
                _ =>{}
            }
        }
    }


    // let my_gltf = asset_server.load("objects/cube1.glb#Scene0");
    // // to position our 3d model, simply use the Transform
    // // in the SceneBundle
    // commands.spawn(SceneBundle {
    //     scene: my_gltf,
    //     transform: Transform::from_xyz(0.0, 0.0, 1.5),
    //     ..Default::default()
    // });

}

fn camera_movement( 
    mut key_evr: EventReader<KeyboardInput>, 
    mut camera: Query<&mut Transform, With<Camera>>,
    mut debug_text: Query<&mut Text, With<components::DebugText>>,
    mut debug_info_res: ResMut<resources::DebugInfo>,
    mut motion_evr: EventReader<MouseMotion>
){

    // camera rotation 
    for ev in motion_evr.iter() {
        let cmr = &mut camera.single_mut();
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        cmr.rotate_y( ev.delta.x.to_radians() * (-0.1) );
        // cmr.rotate_x( ev.delta.y.to_radians() * (-0.1) );
        // cmr.rotate_y( ev.delta.x.to_radians() );
    }


    // camera position 
    for ev in key_evr.iter(){
        if let Some(x) = ev.key_code{
            let cmr = &mut camera.single_mut();

            match x{
                KeyCode::W => { cmr.translation.z += -0.5; },
                KeyCode::S => { cmr.translation.z +=  0.5; },
                KeyCode::A => { cmr.translation.x += -0.5; },
                KeyCode::D => { cmr.translation.x += 0.5; },
                KeyCode::Space => { cmr.translation.y += 0.5; },
                KeyCode::C => { cmr.translation.y += -0.5; },
                // KeyCode::Down => {
                //     let angle =  (-10.0f32).to_radians();
                //     cmr.rotate_x(angle);
                // },
                // KeyCode::Up => { 
                //     let angle =  (10.0f32).to_radians();
                //     cmr.rotate_x(angle);
                // },
                // KeyCode::Left => { 
                //     let angle =  (10.0f32).to_radians();
                //     cmr.rotate_y(angle);
                // },
                // KeyCode::Right => {
                //     let angle =  (-10.0f32).to_radians();
                //     cmr.rotate_y(angle);
                // },
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
