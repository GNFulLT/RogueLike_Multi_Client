use super::audio_manager::*;

pub struct Global
{
    pub audio_manager: AudioManager
}


impl Global 
{
    pub fn new() -> Result<Global,String>
    {
        
        let mut mng = AudioManager::new();
        mng.init();
        
        Ok(Global{ audio_manager:mng })
    }
}