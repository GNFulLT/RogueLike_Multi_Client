pub mod global;
pub mod states;

use global::global::*;
use rltk::RltkBuilder;
use std::rc::Rc;
use states::{main_state::MainState, main_menu_state::MainMenuState};
use std::cell::RefCell;


#[tokio::main]
async fn main() -> rltk::BError {
    let global = Global::new().unwrap();


    let context = RltkBuilder::simple80x50()
        .with_fps_cap(60.)
        .with_title("Roguelike Tutorial")
        .with_font("vga8x16.png",8,16)
        .with_sparse_console(80,25,"vga8x16.png")
        .build()?;

    let state = MainMenuState::new();
    if state.is_none()
    {
        return rltk::BError::Err("Err while trying to create first state".into());
    }
    
    let main_state = MainState::new(Rc::new(RefCell::new(global)),Rc::new(RefCell::new(state.unwrap())));
    rltk::main_loop(context, main_state)

    // if let Some(audio_manager) = global.audio_manager.as_mut()
    // {
    //     println!("There is an audio manager");
    //     let mut need_handle = audio_manager.needs_handle();
    //     let clone = audio_manager.bg_track.shared.clone();
    //     let snk = audio_manager.bg_track.sink.clone();
    //     let buff = audio_manager.resources[AudioManager::MAIN_BG_MUSIC].clone();
    //     let track_data = audio_manager.bg_track.track_data.clone();

    //     tokio::spawn(async move {
    //         let lock = &mut clone.lock().unwrap();
    //         lock.start_sound_and_sleep(buff, snk,track_data);
    //     });
        
    //     while !need_handle
    //     {
    //         need_handle = audio_manager.needs_handle();
    //     }

    //     if need_handle
    //     {
    //         audio_manager.bg_track.stop_sound();
    //         audio_manager.rebuild_output();
    //     }

    //     need_handle = audio_manager.needs_handle();

    //     if !need_handle
    //     {
    //         println!("Hnadled, try to begin sound again");
    //         let clone = audio_manager.bg_track.shared.clone();
    //         let snk = audio_manager.bg_track.sink.clone();
    //         let track_data = audio_manager.bg_track.track_data.clone();

    //         tokio::spawn(async move {
    //             let lock = &mut clone.lock().unwrap();
    //             // let play_time = lock.calculate_play_time();
    //             // println!("Continue from : {}",play_time.as_secs_f32().to_string());
    //             lock.try_continue_and_sleep(snk,track_data);
    //         });
    //     }
    //     loop
    //     {
    //         need_handle = audio_manager.needs_handle();
    //         if need_handle
    //         {
    //             println!("Need handle");
    //             if audio_manager.bg_track.is_playing()
    //             {
    //                 println!("Sound already playing stop, rebuild and continue");

    //                 audio_manager.bg_track.stop_sound();
    //                 audio_manager.rebuild_output();
    //                 let bg_track = audio_manager.bg_track.shared.clone();
    //                 let snk = audio_manager.bg_track.sink.clone();
    //                 let track_data = audio_manager.bg_track.track_data.clone();
    //                 tokio::spawn(async move {
    //                     let lock = &mut bg_track.lock().unwrap();
    //                     let play_time = lock.calculate_play_time();
    //                     println!("Continue from : {}",play_time.as_secs_f32().to_string());
    //                     lock.try_continue_and_sleep(snk,track_data);
    //                 });
    //             }
    //             else
    //             {
    //                 audio_manager.rebuild_output();
    //             }
                
    //         }
    //         else if audio_manager.bg_track.last_sound_finished()
    //         {
    //             println!("Sound finished replay");

    //             let clone = audio_manager.bg_track.shared.clone();
    //             let snk = audio_manager.bg_track.sink.clone();
    //             let buff = audio_manager.resources[AudioManager::MAIN_BG_MUSIC].clone();
    //             let track_data = audio_manager.bg_track.track_data.clone();
    //             tokio::spawn(async move {
    //                 let lock = &mut clone.lock().unwrap();
    //                 lock.start_sound_and_sleep(buff, snk,track_data);
    //             });
    //         }
    //         else
    //         {
    //         }
    //     }
    // }
    // else
    // {
    //     println!("There is no audio manager")
    // }
}   
