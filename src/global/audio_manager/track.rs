use core::panic;
use std::{time::{SystemTime, UNIX_EPOCH, Duration}, sync::{Arc, Mutex}, ops::Deref};
use rodio::{Decoder,source::{Buffered}, Sink, Source};
use std::io::BufReader;
use std::fs::File;

pub struct TrackInfo
{
    current_playing:bool,
    is_finished:bool,
    start_milli : Duration,
    pause_start_milli : Duration,
    pause_end_milli:Duration,
    pause_duration:Duration,
    last_track_start_buffer:Option<Buffered<Decoder<BufReader<File>>>>
}


#[derive(Clone)]
pub struct Track
{
    pub track_info: Arc<Mutex<TrackInfo>>,
    pub sink: Arc<Option<Sink>>,

}

impl Track
{
    pub fn new(sink: Option<Sink>) -> Track
    {
        Track{sink:Arc::new(sink),track_info:Arc::new(Mutex::new(TrackInfo { current_playing: false, is_finished: true, start_milli:Duration::new(0,0), pause_start_milli: Duration::new(0,0), pause_end_milli: Duration::new(0,0), pause_duration: Duration::new(0,0), last_track_start_buffer:None }))}
    }

    pub fn stop_sound(&self)
    {
        if let Some(sink) = self.sink.as_ref()
        {
            sink.stop()
        }
    }

    pub fn is_playing(&self) -> bool
    {
        let mut track_info: std::sync::MutexGuard<'_, TrackInfo> = self.track_info.lock().expect("Couldn't lock the track data");
        track_info.current_playing
    }

    pub fn last_sound_finished(&self) -> bool
    {
        match self.track_info.lock()
        {
            Ok(lock) => 
            {   
                lock.is_finished
            }
            Err(_err) => { panic!("Multithread expcetion")  }
        }
    }

    pub fn can_continue_last_song(&self) -> bool
    {
        let mut track_info: std::sync::MutexGuard<'_, TrackInfo> = self.track_info.lock().expect("Couldn't lock the track data");

        track_info.is_finished
    }

    pub fn try_to_continue_and_sleep(&self)
    {
        if let Some(sink) = self.sink.as_ref()
        {
            let s;
            {
                let mut track_info: std::sync::MutexGuard<'_, TrackInfo> = self.track_info.lock().expect("Couldn't lock the track data");
                if track_info.is_finished
                {
                    return;
                }
                track_info.current_playing = true;
                let dump_buff: Buffered<Decoder<BufReader<File>>> = track_info.last_track_start_buffer.clone().unwrap();
                // start
                // -- +
                // end
                // ---------+
                // pause start
                // ---------+
                // pause end and continue
                // ---------------+
                let curr = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
                track_info.pause_end_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
                let delta = track_info.pause_end_milli - track_info.pause_start_milli;
                track_info.pause_duration += delta;
                let playtime = (curr - track_info.start_milli)-(track_info.pause_duration);
                s = dump_buff.skip_duration(playtime);     
            }
            sink.append(s);
            sink.sleep_until_end();

            {
                let mut track_info = self.track_info.lock().expect("Couldn't lock the track data");
                track_info.current_playing = false;
                track_info.pause_start_milli =SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
                if !track_info.last_track_start_buffer.clone().expect("Couldn't get sound wtf").skip_duration(track_info.calculate_play_time()).clone().next().is_some()
                {
                    track_info.is_finished = true;
                    return;
                }
            }
        }
        
    }

    pub fn start_sound_and_sleep(&self,buffer :Buffered<Decoder<BufReader<File>>>)
    {
        if let Some(sink) = self.sink.as_ref()
        {
            {
                let mut track_info = self.track_info.lock().expect("Couldn't lock the track data");
                track_info.current_playing = true;
                track_info.is_finished = false;

                track_info.pause_end_milli = Duration::new(0,0);
                track_info.pause_start_milli = Duration::new(0,0);
                track_info.start_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
                track_info.last_track_start_buffer = Some(buffer.clone());
            }
            
            sink.append(buffer);
            sink.sleep_until_end();
            
            {
                let mut track_info = self.track_info.lock().expect("Couldn't lock the track data");
                track_info.current_playing = false;
                track_info.pause_start_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
                if !track_info.last_track_start_buffer.clone().expect("Couldn't get sound wtf").skip_duration(track_info.calculate_play_time()).clone().next().is_some()
                {
                    track_info.is_finished = true;
                    return;
                }
            }
        }
        else
        {
            return;
        }
    }
}

impl TrackInfo
{
     pub fn calculate_play_time(&self) -> Duration
     {
         let curr = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
         let pause_end_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
         let total_pause_dir = self.pause_duration + (pause_end_milli - self.pause_start_milli);
         (curr - self.start_milli)-(total_pause_dir)
     }
}

// impl TrackShared{
//     pub fn start_sound_and_sleep(&mut self,buffer:Buffered<Decoder<BufReader<File>>>,sink : Arc<Sink>,track_data:Arc<Mutex<TrackData>>)
//     {
//         match &mut track_data.lock()
//         {
//             Ok(lock) => 
//             {   
//                 lock.is_finished = false;
//                 lock.current_playing = true;  
//             }
//             Err(_err) => {}
//         }
       
//         self.pause_end_milli = Duration::new(0,0);
//         self.pause_start_milli = Duration::new(0,0);
//         self.start_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
//         self.last_track_start_buffer = Some(buffer.clone());


//         sink.append(buffer);
//         sink.sleep_until_end();

//         let lock1 = &mut track_data.lock().unwrap();
//         lock1.current_playing = false;

//         self.pause_start_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");


//         if !self.last_track_start_buffer.clone().expect("Couldn't get sound wtf").skip_duration(self.calculate_play_time()).clone().next().is_some()
//         {
//             lock1.is_finished = true;
//             return;
//         }
//     }

//     pub fn calculate_play_time(&self) -> Duration
//     {
//         let curr = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
//         let pause_end_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
//         let total_pause_dir = self.pause_duration + (pause_end_milli - self.pause_start_milli);
//         (curr - self.start_milli)-(total_pause_dir)
//     }
//     // pub fn stop_sound(&self)
//     // {
//     //     self.sink.stop()
//     // }
//     pub fn try_continue_and_sleep(&mut self,sink : Arc<Sink>,track_data:Arc<Mutex<TrackData>>)
//     {
//         match &mut track_data.lock()
//         {
//             Ok(lock) => 
//             { 

//                 if lock.is_finished
//                 {
//                     return;
//                 }
               
//             }
//             Err(_err) => {  panic!("Multithread exception") }
//         }
//         let dump_buff: Buffered<Decoder<BufReader<File>>> = self.last_track_start_buffer.clone().unwrap();
//         // start
//         // -- +
//         // end
//         // ---------+
//         // pause start
//         // ---------+
//         // pause end and continue
//         // ---------------+
//         let curr = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
//         self.pause_end_milli = SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
//         self.pause_duration += self.pause_end_milli - self.pause_start_milli;
//         let playtime = (curr - self.start_milli)-(self.pause_duration);
//         let s = dump_buff.skip_duration(playtime);
       
//         sink.append(s);
//         sink.sleep_until_end();
//         let lock1 = &mut track_data.lock().unwrap();
//         lock1.current_playing = false;
//         self.pause_start_milli =SystemTime::now().duration_since(UNIX_EPOCH).expect("Couldn't get duration");
//         if !self.last_track_start_buffer.clone().expect("Couldn't get sound wtf").skip_duration(self.calculate_play_time()).clone().next().is_some()
//         {
//             lock1.is_finished = true;
//             return;
//         }
//     }

// }