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
        let re = Regex::new(&regex).expect("regex invalid");
        songs.retain(|i| re.is_match(i));
    }

    let mut playlist = playlist::Playlist::new(songs);

    let mut ui = UI::new(&playlist);

    gst::init();
    let mut playbin = gst::PlayBin::new("audio_player")
        .expect("Couldn't create playlist");
    let mut main_loop = gst::MainLoop::new();

    let mut bus;
    let bus_receiver;

    let song = match playlist.get_next_song() {
        Some(a) => gst::filename_to_uri(a).expect("filename error"),
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
                            panic!("error msg from element `{}`: {}, quit",
                                     message.src_name(), error.message());
                        }
                        gst::Message::Eos(ref _msg) => {
                            break 'outer;
                        }
                        gst::Message::StreamStart(ref _msg) => {
                            song_buffered = false;
                            playlist.go_to_next();
                        }
                        _ => {
                            //println!("msg of type `{}` from element `{}`",
                            //         message.type_name(), message.src_name());
                        }
                    }
                }
                Err(err) => {
                    match err {
                        std::sync::mpsc::TryRecvError::Empty => break,
                        std::sync::mpsc::TryRecvError::Disconnected => break,
                    }
                }
            }
        }
        let stream_dir = playbin.duration_s();
        let stream_pos = playbin.position_s();

        if stream_dir.is_some() && stream_pos.is_some() && !song_buffered {
            if stream_dir.unwrap() - stream_pos.unwrap() < 3.0 {
                match playlist.get_next_song() {
                    Some(a) => {
                        let song = gst::filename_to_uri(a).expect("URI error");
                        playbin.set_uri(&song);
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

        match ui.manage_ui(&playlist, stream_pos, stream_dir) {
            UIResult::PlayPause => {
                println!("play/pause");
            }
            UIResult::Previous => {
                println!("previous");
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
        }
    }
    main_loop.quit();
}
