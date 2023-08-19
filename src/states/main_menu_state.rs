use std::{collections::{VecDeque, HashMap}, fs::File, io::BufReader};

use rltk::GameState;
use rodio::{source::Buffered, Decoder,Source};

use super::rg_state::RgState;

pub struct MainMenuState
{
    menu_stack:VecDeque<Box<dyn RgState>>,
    bg_music :Buffered<Decoder<BufReader<File>>>,
}

impl MainMenuState
{
    pub fn new() -> Option<MainMenuState>
    {
        let file_res = File::open("./assets/sounds/main_bg.mp3");
        if file_res.is_err()
        {
            println!("Couldn't read the asset");
            return None;
        }

        let file: BufReader<File> = BufReader::new(file_res.unwrap());
        let decoder_res = rodio::Decoder::new(file);
        
        if decoder_res.is_err()
        {
            println!("Given file is not mp4");
            return None;
        }
        Some(MainMenuState{menu_stack:VecDeque::new(),bg_music:decoder_res.unwrap().buffered()})
    }
}

impl GameState for MainMenuState
{
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        
    }
}

impl RgState for MainMenuState
{
    fn on_init(&mut self,props:&super::rg_state::InitProps) -> bool {

        props.global.borrow_mut().audio_manager.play_bg_track_async(self.bg_music.clone());
        return true;  
    }

    fn on_quit(&mut self,quit:&super::rg_state::QuitProps) {
        
    }
}