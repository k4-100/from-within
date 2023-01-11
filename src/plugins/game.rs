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
        .add_system(camera_mouse_movement)
        .add_system(camera_keyboard_movement)
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
                        transform: Transform::from_xyz(3.0 * b as f32, 0., 3.0 * a as f32 + 1.5)
                            // .looking_at( Vec3::new(0.,0.,0.), Vec3::new(0.0,1.0,0.))
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

fn camera_mouse_movement( 
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut debug_text: Query<&mut Text, With<components::DebugText>>,
    mut debug_info_res: ResMut<resources::DebugInfo>,
    mut motion_evr: EventReader<MouseMotion>
){
    let cmr = &mut camera.single_mut();

    // camera rotation 
    for ev in motion_evr.iter() {
        // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        cmr.rotate_local_y( ev.delta.x.to_radians() * (-0.1) );
        println!("MM");
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

fn  camera_keyboard_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
){

    let cmr = &mut camera.single_mut();
    let mut velocity = Vec3::ZERO;
    let local_z = cmr.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    // camera position 
    for key in keys.get_pressed(){
        match key{
            KeyCode::W => velocity += forward,
            KeyCode::S => velocity -= forward,
            KeyCode::A => velocity -= right,
            KeyCode::D => velocity += right,
            _ => {}
        }

        velocity.normalize_or_zero();
        cmr.translation += velocity * time.delta_seconds() * 2.0;
        println!("MK");
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
