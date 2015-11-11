//  Copyright 2015 Nathanael Merrill
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
extern crate rustty;
extern crate gst;
extern crate argparse;
extern crate regex;

mod playlist;
mod default_ui;

use argparse::{ArgumentParser, Store, List};
use default_ui::*;
use regex::Regex;
use gst::ElementT;

/*fn main_loop(playlist: &mut playlist::Playlist, ui: &mut UI) {
    let mut song_done = true;
    let mut adder = 0.01;
    let mut song: String = String::from("");
    let mut time = 0.0_f32;
    let mut paused = false;
    let mut song_nevermind = false;
    //let mut time = 0_f32;
    let index = 0;
    loop {
        if song_done {
            song = match playlist.get_next_song() {
                Some(a) => a.to_string(),
                None => break,
            };
            song_done = false;
            time = 0.0;
        } else if song_nevermind {
            song = match playlist.get_prev_song() {
                Some(a) => a.to_string(),
                None => break,
            };
            song_nevermind = false;
            time = 0.0;
        }
        time = time + adder;
        
        let totaltime = 150;
        let commitTime = time.round() as i32;
        /*let commitTime = 35;
        
        let totaltime = 150;*/
        if commitTime == totaltime - 1 {
            song_done = true;
        }
        match ui.manage_ui(song.to_string(), commitTime, totaltime) {
            UIResult::Play => {
                adder = 0.01;
            }
            UIResult::PlayPause => {
                paused = !paused;
                match paused {
                    true => adder = 0.0,
                    false => adder = 0.01,
                };
            }
            UIResult::Pause => {
                adder = 0.0;
            }
            UIResult::Previous => {
                song_nevermind = true;
            }
            UIResult::Next => {
                //println!("next");
                song_done = true;
            }
            UIResult::Exit => {
                println!("exit");
                break;
            }
            UIResult::Error => {
                println!("error");
                break;
            }
            UIResult::NA => {}
        }
    }
}*/

fn main() {
    let mut regex = String::new();
    let mut songs : Vec<String> = vec![]; // TODO: add with capacity!
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Rust Fucking Simple Music Player");
        ap.refer(&mut regex)
            .add_option(&["-r", "--regex"], Store, "Use a regual expression");
        ap.refer(&mut songs)
            .add_argument("arguments", List, "Songs to play");
        ap.parse_args_or_exit();
    }

    if regex != "" {
        let re = Regex::new(&regex).unwrap();
        songs.retain(|i| re.is_match(i));
    }
    else {
        // TODO
    }

    let mut playlist = playlist::Playlist::new(songs.clone());

    let mut ui = UI::new(songs);

    gst::init();
    let mut playbin = gst::PlayBin::new("audio_player")
        .expect("Couldn't create playlist");
    let mut main_loop = gst::MainLoop::new();

    let mut bus;
    let bus_receiver;

    let song = match playlist.get_next_song() {
        Some(a) => gst::filename_to_uri(a).unwrap(),
        None => panic!("can't get song"),
    };
    playbin.set_uri(&song);
    bus = playbin.bus().expect("Couldn't get pipeline bus");
    bus_receiver = bus.receiver();
    main_loop.spawn();
    playbin.play();

    let mut song_buffered = false;

    'outer: loop {
        loop {
            match bus_receiver.try_recv() {
                Ok(message) => {
                    match message.parse(){
                        gst::Message::ErrorParsed{ref error, ..} => {
                            println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
                            break;
                        }
                        gst::Message::Eos(ref _msg) => {
                            println!("eos received quiting");
                            break 'outer;
                        }
                        gst::Message::StreamStart(ref _msg) => {
                            song_buffered = false;
                        }
                        /*gst::Message::DurationChanged(ref msg) => {
                            let stream_dir = playbin.duration_s();
                            let stream_pos = playbin.position_s();

                            if stream_dir.is_some() && stream_pos.is_some() {
                                println!("{}", stream_pos.unwrap());
                                println!("{}", stream_dir.unwrap());
                            }
                        }*/
                        _ => {
                            //println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
                        }
                    }
                }
                Err(err) => {
                    match err {
                        std::sync::mpsc::TryRecvError::Empty => break,
                        std::sync::mpsc::TryRecvError::Disconnected => println!("Dist"),
                    }
                }
            }
        }
        let stream_dir = playbin.duration_s();
        let stream_pos = playbin.position_s();

        if stream_dir.is_some() && stream_pos.is_some() && !song_buffered {
            //println!("{}", stream_pos.unwrap());
            //println!("{}", stream_dir.unwrap());
            if stream_dir.unwrap() - stream_pos.unwrap() < 3.0 {
                //println!("{}", stream_pos.unwrap());
                //println!("{}", stream_dir.unwrap());
                match playlist.get_next_song() {
                    Some(a) => {
                        //println!("making song");
                        let song = gst::filename_to_uri(a).unwrap();
                        //println!("{}", song);
                        playbin.set_uri(&song);
                        //println!("done making song");
                        song_buffered = true;
                    }
                    None => {
                        //println!("All songs played");
                    }
                };
            }
        }

        let stream_dir = match stream_dir {
            Some(a) => a as i32,
            None => 0,
        };
        let stream_pos = match stream_pos {
            Some(a) => a as i32,
            None => 0,
        };

        match ui.manage_ui(song.to_string(), stream_pos, stream_dir) {
            UIResult::Play => {
                //println!("play");
            }
            UIResult::Pause => {
                println!("pause");
            }
            UIResult::Next => {
                println!("next");
            }
            UIResult::Exit => {
                println!("exit");
                break;
            }
            UIResult::Error => {
                println!("error");
                break;
            }
            UIResult::NA => {}
            _ => println!{"Not implemented"},
        }
    }
    main_loop.quit();
}
