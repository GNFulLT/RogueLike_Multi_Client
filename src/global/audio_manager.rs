use rodio::{OutputStream,OutputStreamHandle,Decoder,source::{Buffered}, Sink, Device, cpal::{Host}};
use std::{collections::HashMap, sync::Mutex, borrow::BorrowMut};
use std::io::BufReader;
use std::fs::File;
use rodio::cpal::traits::{HostTrait,DeviceTrait};
use rodio::*;

use self::track::Track;
use std::sync::Arc;
mod track;

pub struct AudioManager
{
    host : Host,
    device : Option<Device>,
    output_stream : Option<OutputStream>,
    pub output_stream_handle : Option<OutputStreamHandle>,
    pub bg_track : Track
}


impl AudioManager
{
    pub const MAIN_BG_MUSIC : &str = "MAIN_BG_MUSIC";
 
    pub fn can_be_created() -> bool
    {
        let host = cpal::default_host();
        host.devices().unwrap().count() != 0
    }

    pub fn any_selected_device(&self) ->bool
    {
        if let Some(dev) = &self.device
        {
            return true;
        }
        return false;
    }

    pub fn new() -> AudioManager
    {
        let host = cpal::default_host();
        let device = host.default_output_device();
        let mut bg_track: Track = Track::new(None);
        let mut output_stream =None;
        let mut output_stream_handle = None;
        
        if device.is_some()
        {
            let (toutput_stream,toutput_stream_handle) = OutputStream::try_from_device(device.as_ref().unwrap()).expect("Couldn't create audio stream");
            output_stream = Some(toutput_stream);
            output_stream_handle = Some(toutput_stream_handle);

            bg_track.sink = Arc::new(Some(Sink::try_new(output_stream_handle.as_ref().unwrap()).unwrap()));  
        }
        AudioManager{bg_track, output_stream,output_stream_handle,host,device}
    }

    pub fn init(&mut self) 
    {
        let file: BufReader<File> = BufReader::new(File::open("./assets/sounds/main_bg.mp3").unwrap());
        // Decode that sound file into a source
        //self.resources.insert(AudioManager::MAIN_BG_MUSIC.to_string(), rodio::Decoder::new(file).unwrap().buffered());
     
    }

    pub fn available_device(&self) -> bool
    {
        match self.host.devices()
        {
            Ok(devices) => { devices.count() != 0 }
            Err(err) => {println!("{}",err.to_string());return false;}
        }
        
    }

    pub fn play_bg_track_async(&mut self,music_data :Buffered<Decoder<BufReader<File>>>) -> bool
    {
        let bg_track = self.bg_track.clone();
        tokio::spawn(async move {
            bg_track.start_sound_and_sleep(music_data);
        });

        return true;
    }   

    pub fn try_to_continue_bg_async(&mut self) -> bool
    {
        let bg_track = self.bg_track.clone();
        println!("Continue");
        tokio::spawn(async move {
            bg_track.try_to_continue_and_sleep();
        });
        return true;
    }

    pub fn rebuild_output(&mut self)
    {
        let device = self.host.default_output_device();
        if let Some(dev) = device
        {
            self.device = Some(dev);
            

            let (output_stream,output_stream_handle) = OutputStream::try_from_device(&self.device.as_ref().unwrap()).expect("Couldn't create audio stream");
            

            self.output_stream = Some(output_stream);
            
            self.output_stream_handle = Some(output_stream_handle);
            
            self.bg_track.borrow_mut().sink = Arc::new(Some(Sink::try_new(&self.output_stream_handle.as_ref().unwrap()).unwrap()));
        }
        // Create sinks 

    }

    pub fn needs_handle(&self) -> bool
    {
        let dev = self.host.default_output_device();
        if let Some(device) = dev
        {
            if let Some(self_device) = &self.device
            {
                if self_device.name().unwrap() == device.name().unwrap()
                {
                    return false;
                }
                else 
                {
                    return true;
                }
            }
            else {
                return true;
            }
            
        }
        else
        {
            return true;
        }
    }
}