use rodio::{OutputStream,OutputStreamHandle,Decoder,source::{Buffered}, Sink, Device, Devices, cpal::{Host, StreamConfig}};
use std::{collections::HashMap};
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
    device : Device,
    output_stream : OutputStream,
    pub output_stream_handle : OutputStreamHandle,
    pub resources : HashMap<String,Buffered<Decoder<BufReader<File>>>>,
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
    pub fn new() -> AudioManager
    {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        
        let (output_stream,output_stream_handle) = OutputStream::try_from_device(&device).expect("Couldn't create audio stream");
        let bg_sink = Sink::try_new(&output_stream_handle).unwrap();  
        let bg_track = Track::new(bg_sink);
        AudioManager{bg_track, output_stream,output_stream_handle,host,device,resources:HashMap::new() }
    }

    pub fn init(&mut self) 
    {
        let file = BufReader::new(File::open("./assets/sounds/main_bg.mp3").unwrap());
        // Decode that sound file into a source
        self.resources.insert(AudioManager::MAIN_BG_MUSIC.to_string(), rodio::Decoder::new(file).unwrap().buffered());
     
    }

    pub fn available_device(&self) -> bool
    {
        match self.host.devices()
        {
            Ok(devices) => { devices.count() != 0 }
            Err(err) => {println!("{}",err.to_string());return false;}
        }
        
    }

    pub fn rebuild_output(&mut self)
    {
        let device = self.host.default_output_device().expect("Couldn't handle audio device");
        self.device = device;
        let (output_stream,output_stream_handle) = OutputStream::try_from_device(&self.device).expect("Couldn't create audio stream");
        self.output_stream = output_stream;
        self.output_stream_handle = output_stream_handle;

        // Create sinks 

        self.bg_track.sink = Arc::new(Sink::try_new(&self.output_stream_handle).unwrap());
    }

    pub fn needs_handle(&self) -> bool
    {
        let dev = self.host.default_output_device();
        if let Some(device) = dev
        {
            if self.device.name().unwrap() == device.name().unwrap()
            {
                return false;
            }
            else 
            {
                return true;
            }
        }
        else
        {
            return true;
        }
    }
}