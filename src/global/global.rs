use super::audio_manager::*;

pub struct Global
{
    pub audio_manager: Option<AudioManager>
}


impl Global 
{
    pub fn new() -> Global
    {
        let mut audio_manager:Option<AudioManager> = None;
        if AudioManager::can_be_created()
        {
            let mut mng = AudioManager::new();
            mng.init();
            audio_manager = Some(mng);
        }
        
        Global{ audio_manager }
    }
}