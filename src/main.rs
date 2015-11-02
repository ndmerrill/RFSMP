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

extern crate argparse;

mod playlist;

use std::io;
use argparse::{ArgumentParser, Store, List};

enum UIResult {
    Play,
    Pause,
    Exit,
    Error,
    NA,
}

fn manage_ui() -> UIResult {
    let stdin = io::stdin();

    let mut input = String::new();
    let read;

    read = stdin.read_line(&mut input);
    if read.is_err() {
        return UIResult::Error;
    }

    match &*input {
        "l\n" => UIResult::Play,
        "p\n" => UIResult::Pause,
        "x\n" => UIResult::Exit,
        _ => {
            println!("Unknown Command");
            UIResult::NA
        }
    }
}

fn main_loop(playlist: &mut playlist::Playlist) {
    let mut song_done = true;
    loop {
        if song_done == true {
            let song = match playlist.get_next_song() {
                Some(a) => a,
                None => break,
            };
            song_done = false;
        }

        match manage_ui() {
            UIResult::Play => {
                println!("play");
            }
            UIResult::Pause => {
                println!("pause");
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

    songs = vec![String::from("/home/nathan/Music/Brite Futures/Glistening Pleasure/12 The Malibu Highlife.m4a"),
                 String::from("/home/nathan/Music/Brite Futures/Glistening Pleasure 2.0/04 - Iceage Babeland.mp3")];

    let mut playlist = playlist::Playlist::new(songs);

    println!("Commands:");
    println!("\tPlay : l");
    println!("\tPause: p");
    println!("\tExit : x");

    main_loop(&mut playlist);
}
