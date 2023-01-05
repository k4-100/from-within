use bevy::prelude::*;

// region: --- PlayerPlugin Components 

#[derive(Resource)]
pub struct DebugInfo {
    pub camera_transform: String,
    pub fps: f64
}

impl DebugInfo {
    pub fn get_formatted_debug_info(&self) -> String{
        format!( "INFO:\n{}\nFPS:{}", self.camera_transform, self.fps )
    }
}
// endregion: --- PlayerPlugin Components 
