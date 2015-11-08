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
extern crate argparse;
extern crate regex;

mod playlist;
mod default_ui;

use argparse::{ArgumentParser, Store, List};
use default_ui::*;
use regex::Regex;

fn main_loop(playlist: &mut playlist::Playlist, ui: &mut UI) {
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
}

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

    //songs = vec![String::from("/home/nathan/Music/Brite Futures/Glistening Pleasure/12 The Malibu Highlife.m4a"),
   // String::from("/home/nathan/Music/Brite Futures/Glistening Pleasure 2.0/04 - Iceage Babeland.mp3")];

    let mut playlist = playlist::Playlist::new(songs.clone());

    let mut ui = UI::new(songs);

    main_loop(&mut playlist, &mut ui);
}
