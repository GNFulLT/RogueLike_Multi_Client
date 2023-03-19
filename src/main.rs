pub mod global;

use std::f32::consts::E;

use global::{global::*,audio_manager::*};
use rodio::Source;

#[tokio::main]
async fn main() {
    let mut global = Global::new();

    if let Some(audio_manager) = global.audio_manager.as_mut()
    {
        println!("There is an audio manager");
        let mut need_handle = audio_manager.needs_handle();
        let clone = audio_manager.bg_track.shared.clone();
        let snk = audio_manager.bg_track.sink.clone();
        let buff = audio_manager.resources[AudioManager::MAIN_BG_MUSIC].clone();
        let track_data = audio_manager.bg_track.track_data.clone();

        tokio::spawn(async move {
            let lock = &mut clone.lock().unwrap();
            lock.start_sound_and_sleep(buff, snk,track_data);
        });
        
        while !need_handle
        {
            need_handle = audio_manager.needs_handle();
        }

        if need_handle
        {
            audio_manager.bg_track.stop_sound();
            audio_manager.rebuild_output();
        }

        need_handle = audio_manager.needs_handle();

        if !need_handle
        {
            println!("Hnadled, try to begin sound again");
            let clone = audio_manager.bg_track.shared.clone();
            let snk = audio_manager.bg_track.sink.clone();
            let track_data = audio_manager.bg_track.track_data.clone();

            tokio::spawn(async move {
                let lock = &mut clone.lock().unwrap();
                // let play_time = lock.calculate_play_time();
                // println!("Continue from : {}",play_time.as_secs_f32().to_string());
                lock.try_continue_and_sleep(snk,track_data);
            });
        }
        loop
        {
            need_handle = audio_manager.needs_handle();
            if need_handle
            {
                println!("Need handle");
                if audio_manager.bg_track.is_playing()
                {
                    println!("Sound already playing stop, rebuild and continue");

                    audio_manager.bg_track.stop_sound();
                    audio_manager.rebuild_output();
                    let bg_track = audio_manager.bg_track.shared.clone();
                    let snk = audio_manager.bg_track.sink.clone();
                    let track_data = audio_manager.bg_track.track_data.clone();
                    tokio::spawn(async move {
                        let lock = &mut bg_track.lock().unwrap();
                        let play_time = lock.calculate_play_time();
                        println!("Continue from : {}",play_time.as_secs_f32().to_string());
                        lock.try_continue_and_sleep(snk,track_data);
                    });
                }
                else
                {
                    audio_manager.rebuild_output();
                }
                
            }
            else if audio_manager.bg_track.last_sound_finished()
            {
                println!("Sound finished replay");

                let clone = audio_manager.bg_track.shared.clone();
                let snk = audio_manager.bg_track.sink.clone();
                let buff = audio_manager.resources[AudioManager::MAIN_BG_MUSIC].clone();
                let track_data = audio_manager.bg_track.track_data.clone();
                tokio::spawn(async move {
                    let lock = &mut clone.lock().unwrap();
                    lock.start_sound_and_sleep(buff, snk,track_data);
                });
            }
            else
            {
            }
        }
    }
    else
    {
        println!("There is no audio manager")
    }
}   
