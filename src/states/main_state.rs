use std::{fs::File, borrow::BorrowMut};
use std::io::BufReader;
use std::rc::Rc;
use std::cell::RefCell;
use rltk::GameState;
use rodio::Decoder;
use rodio::source::Buffered;

use crate::global::global::Global;
use super::rg_state::{RgState, InitProps};
pub struct MainState
{
    pub global:Rc<RefCell<Global>>,
    pub current_state: Rc<RefCell<dyn RgState>>,
    state_inited: bool
}

impl MainState
{
    pub fn new(global:Rc<RefCell<Global>>,first_state:Rc<RefCell<dyn RgState>>) -> MainState
    {
        MainState { global,current_state:first_state ,state_inited:false}
    }

    pub fn begin_bg_track(&mut self,buff:Buffered<Decoder<BufReader<File>>>)
    {       
        let audio_manager = &mut self.global.as_ref().borrow_mut().audio_manager; 
        audio_manager.play_bg_track_async(buff);
    }
}

impl GameState for MainState
{
    fn tick(&mut self, ctx: &mut rltk::BTerm) {

        if !self.state_inited
        {
            let init_props = InitProps{ctx,global:self.global.clone()};
            self.state_inited = self.current_state.as_ref().borrow_mut().on_init(&init_props);
        }

        // First audio check
        let audio_manager = &mut self.global.as_ref().borrow_mut().audio_manager;
        
            let need_handle = audio_manager.needs_handle();

            // Output device changed Need handling
            if need_handle
            {
                println!("Need handle");
                // There is an already music that is playing. Stop it rebuild the output and continue
                if audio_manager.bg_track.is_playing()
                {
                    println!("Sound already playing stop, rebuild and continue");
                    audio_manager.bg_track.stop_sound();
                    
                    audio_manager.rebuild_output();
                    
                    audio_manager.try_to_continue_bg_async();
                }
                // There is no music that is playing. Just rebuild the output
                else {
                    audio_manager.rebuild_output();
                }
            }
            
                

    }
}

