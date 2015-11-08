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

extern crate gst;
extern crate argparse;

mod playlist;
mod default_ui;

use argparse::{ArgumentParser, Store, List};
use default_ui::*;

use gst::ElementT;

fn main() {
    let mut regex = "".to_string();
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
        println!("Regex not implemented");
        return;
    }
    else {
        // TODO
    }

    //songs = vec![String::from("/home/nathan/Music/Brite Futures/Glistening Pleasure/12 The Malibu Highlife.m4a"),
                 //String::from("/home/nathan/Music/Brite Futures/Glistening Pleasure 2.0/04 - Iceage Babeland.mp3")];

    println!("{}", songs[0]);
    let mut playlist = playlist::Playlist::new(songs);

    let ui = UI::new();

    gst::init();
    let mut playbin = gst::PlayBin::new("audio_player")
        .expect("Couldn't create playlist");
    let mut main_loop = gst::MainLoop::new();

    let mut bus;
    let mut bus_receiver;

    let song = match playlist.get_next_song() {
        Some(a) => gst::filename_to_uri(a).unwrap(),
        None => panic!("can't get song"),
    };
    playbin.set_uri(&song);
    bus = playbin.bus().expect("Couldn't get pipeline bus");
    bus_receiver = bus.receiver();
    main_loop.spawn();
    playbin.play();

    loop {
        loop {
            match bus_receiver.try_recv() {
                Ok(message) => {
                    match message.parse(){
                        gst::Message::StateChangedParsed{ref msg, ref old, ref new, ref pending} => {
                            //println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
                        }
                        gst::Message::ErrorParsed{ref msg, ref error, ref debug} => {
                            println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
                            break;
                        }
                        gst::Message::Eos(ref msg) => {
                            println!("eos received quiting");
                            break;
                        }
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


        match ui.manage_ui() {
            UIResult::Play => {
                println!("play");
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
        }
    }
    main_loop.quit();
}
