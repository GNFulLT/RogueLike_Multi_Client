use core::panic;
use std::{time::{SystemTime, UNIX_EPOCH, Duration}, sync::{Arc, Mutex}};
use rodio::{Decoder,source::{Buffered}, Sink, Source};
use std::io::BufReader;
use std::fs::File;

use crate::global::audio_manager::track;


pub struct TrackData
{
    current_playing:bool,
    is_finished:bool

}

pub struct TrackShared
{
    start_milli : Duration,
    pause_start_milli : Duration,
    pause_end_milli:Duration,
    pause_duration:Duration,
    last_track_start_buffer:Option<Buffered<Decoder<BufReader<File>>>>
}

pub struct Track
{
    pub shared:Arc<Mutex<TrackShared>>,
    pub sink: Arc<Sink>,
    pub track_data:Arc<Mutex<TrackData>>

}

impl Track
{
    pub fn new(sink: Sink) -> Track
    {
        let ts = TrackShared{pause_duration:Duration::new(0,0),pause_end_milli:Duration::new(0,0),pause_start_milli:Duration::new(0,0),last_track_start_buffer:None,start_milli:Duration::new(0, 0)};
        Track{sink:Arc::new(sink),shared:Arc::new(Mutex::new(ts)),track_data:Arc::new(Mutex::new(TrackData{is_finished:true,current_playing:false}))}
    }

    pub fn stop_sound(&self)
    {
        self.sink.stop()
    }

    pub fn is_playing(&self) -> bool
    {
        !self.sink.empty()
    }

    pub fn last_sound_finished(&self) -> bool
    {
        match self.track_data.lock()
        {
            Ok(lock) => 
            {   
                lock.is_finished
            }
            Err(err) => { panic!("Multithread expcetion")  }
        }
    }
}

impl TrackShared{
    pub fn start_sound_and_sleep(&mut self,buffer:Buffered<Decoder<BufReader<File>>>,sink : Arc<Sink>,track_data:Arc<Mutex<TrackData>>)
    {
        match &mut track_data.lock()
        {
            Ok(lock) => 
            {   
                lock.is_finished = false;
                lock.current_playing = true;  
            }
            Err(err) => {}
        }
       
        self.pause_end_milli = Duration::new(0,0);
        self.pause_start_milli = Duration::new(0,0);
        self.start_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
        self.last_track_start_buffer = Some(buffer.clone());


        sink.append(buffer);
        sink.sleep_until_end();

        let lock1 = &mut track_data.lock().unwrap();
        lock1.current_playing = false;

        self.pause_start_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");


        if !self.last_track_start_buffer.clone().expect("Couldn't get sound wtf").skip_duration(self.calculate_play_time()).clone().next().is_some()
        {
            lock1.is_finished = true;
            return;
        }
    }

    pub fn calculate_play_time(&self) -> Duration
    {
        let curr = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
        let pause_end_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
        let total_pause_dir = self.pause_duration + (pause_end_milli - self.pause_start_milli);
        (curr - self.start_milli)-(total_pause_dir)
    }
    // pub fn stop_sound(&self)
    // {
    //     self.sink.stop()
    // }
    pub fn try_continue_and_sleep(&mut self,sink : Arc<Sink>,track_data:Arc<Mutex<TrackData>>)
    {
        match &mut track_data.lock()
        {
            Ok(lock) => 
            { 

                if lock.is_finished
                {
                    return;
                }
               
            }
            Err(err) => {  panic!("Multithread exception") }
        }
        let dump_buff = self.last_track_start_buffer.clone().unwrap();
        // start
        // -- +
        // end
        // ---------+
        // pause start
        // ---------+
        // pause end and continue
        // ---------------+
        let curr = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
        self.pause_end_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
        self.pause_duration += self.pause_end_milli - self.pause_start_milli;
        let playtime = (curr - self.start_milli)-(self.pause_duration);
        let s = dump_buff.skip_duration(playtime);
       
        sink.append(s);
        sink.sleep_until_end();
        let lock1 = &mut track_data.lock().unwrap();
        lock1.current_playing = false;
        self.pause_start_milli =SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
        if !self.last_track_start_buffer.clone().expect("Couldn't get sound wtf").skip_duration(self.calculate_play_time()).clone().next().is_some()
        {
            lock1.is_finished = true;
            return;
        }
    }

}