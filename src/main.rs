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

use argparse::{ArgumentParser, Store, StoreTrue, List};
use default_ui::*;
use regex::Regex;
use gst::ElementT;
use std::fs;
use std::io;

fn recurse_songs(songs: &mut Vec<String>, recurse: bool, is_first: bool) ->
                    Result<(), io::Error>{
    let mut new : Vec<String> = vec![];

    for song in songs.iter() {
        if try!(fs::metadata(&song)).is_dir() {
            if recurse || is_first {
                let mut contents: Vec<String> = vec![];
                for entry in try!(fs::read_dir(song)) {
                    let memes = String::from(match try!(entry).path().to_str() {
                        Some(a) => a,
                        None => return Err(io::Error::new(
                                io::ErrorKind::InvalidInput, "Failed to parse song file names.")),
                    });
                    contents.push(memes);
                }
                try!(recurse_songs(&mut contents, recurse, false));
                new.append(&mut contents);
            }
            else {
                println!("WARNING: Ignoring directory {}", song);
                println!("Use -r to make recursive");
                println!("Press Enter to continue");
                let mut temp = String::new();
                match io::stdin().read_line(&mut temp) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
            }
        }
        else {
            new.push(song.clone());
        }
    }
    songs.clear();
    songs.append(&mut new);
    Ok(())
}

fn main() {
    let mut global_err = String::from("");
    {
    let mut regex = String::new();
    let mut songs : Vec<String> = vec![]; // TODO: add with capacity!
    let mut recurse = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Rust Fucking Simple Music Player");
        ap.refer(&mut recurse)
            .add_option(&["-r", "--recursive"], StoreTrue, "Recurse through directories if passed them");
        ap.refer(&mut regex)
            .add_option(&["-s", "--search"], Store, "Search songs using regular expressions");
        ap.refer(&mut songs)
            .add_argument("arguments", List, "Songs to play");
        ap.parse_args_or_exit();
    }

    let mut recur_one = false;
    if songs.len() == 1 {
        recur_one = true;
    }

    match recurse_songs(&mut songs, recurse, recur_one) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }

    if songs.len() == 0 {
        println!("Usage:");
        println!("    rfsmp [OPTIONS] [SONGS ...]");
        return;
    }

    if regex != "" {
        let re = match Regex::new(&regex) {
            Ok(o) => o,
            Err(e) => {
                println!("Regex invalid: {}", e.to_string());
                return;
            }
        };
        songs.retain(|i| re.is_match(i));
        if songs.len() == 0 {
            println!("Failed to find songs matching regex patern.");
            return;
        }
    }

    let mut playlist = playlist::Playlist::new(songs);

    gst::init();
    let mut playbin = match gst::PlayBin::new("audio_player") {
        Some(a) => a,
        None => {
            println!("Couldn't create PlayBin.");
            return;
        }
    };
    let mut main_loop = gst::MainLoop::new();

    let mut bus;
    let bus_receiver;

    let song = match playlist.get_next_song() {
        Some(a) => gst::filename_to_uri(a).expect("URI Error"),
        None => {
            println!("Can't get song.");
            return;
        }
    };

    playbin.set_uri(&song);
    bus = playbin.bus().expect("Couldn't get pipeline bus");
    bus_receiver = bus.receiver();


    let mut ui = UI::new(&playlist);

    main_loop.spawn();
    playbin.play();

    let mut song_buffered = false;

    'outer: loop {
        loop {
            match bus_receiver.try_recv() {
                Ok(message) => {
                    match message.parse(){
                        gst::Message::ErrorParsed{ref error, ..} => {
                            global_err.push_str(
                                &format!("error msg from element `{}`: {}, quit",
                                        message.src_name(), error.message()));
                            break 'outer;
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
                        let song = match gst::filename_to_uri(a) {
                            Ok(a) => a,
                            Err(e) => {
                                global_err.push_str("URI Error ");
                                global_err.push_str(&e.message());
                                break 'outer;
                            }
                        };
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
                if playbin.is_paused() {
                    playbin.play();
                }
                else {
                    playbin.pause();
                }
            }
            UIResult::Previous => {
                playbin.set_state(gst::ffi::GstState::GST_STATE_NULL);
                playlist.go_to_prev();
                playlist.go_to_prev();
                match playlist.get_next_song() {
                    Some(a) => {
                        let song = match gst::filename_to_uri(a) {
                            Ok(a) => a,
                            Err(e) => {
                                global_err.push_str("URI Error ");
                                global_err.push_str(&e.message());
                                break 'outer;
                            }
                        };
                        playbin.set_uri(&song);
                        song_buffered = true;
                    }
                    None => {
                        println!("All songs played");
                        break;
                    }
                };
                playbin.play();

            }
            UIResult::Next => {
                playbin.set_state(gst::ffi::GstState::GST_STATE_NULL);
                match playlist.get_next_song() {
                    Some(a) => {
                        let song = match gst::filename_to_uri(a) {
                            Ok(a) => a,
                            Err(e) => {
                                global_err.push_str("URI Error ");
                                global_err.push_str(&e.message());
                                break 'outer;
                            }
                        };
                        playbin.set_uri(&song);
                        song_buffered = true;
                    }
                    None => {
                        println!("All songs played");
                        break;
                    }
                };
                playbin.play();
            }
            UIResult::Exit => {
                break;
            }
            UIResult::Error(a) => {
                global_err = a;
                break;
            }
            UIResult::NA => {}
        }
        std::thread::sleep(std::time::Duration::new(0, 20000000));
    }
    main_loop.quit();
    }
    if global_err != "" {
        println!("{}", global_err);
    }
}
