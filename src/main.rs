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

extern crate rfmod;
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

fn init_fmod() -> Result<rfmod::Sys, rfmod::Result> {
    let fmod = try!(rfmod::Sys::new());

    match fmod.init() {
        rfmod::Result::Ok => Ok(fmod),
        e => Err(e),
    }
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

fn main_loop(rfmod: &rfmod::Sys, playlist: &mut playlist::Playlist) -> rfmod::Result {
    loop {
        let song = match playlist.get_next_song() {
            Some(a) => String::from("/home/nathan/Downloads/right.mp3"),
            None => break,
        };
        println!("{}", song);
        let sound = match rfmod.create_sound(&song, None, None) {
            Ok(f) => f,
            Err(e) => {
                println!("Make sound error {:?}", e);
                return e;
            }
        };

        let chan = match sound.play() {
            Ok(f) => f,
            Err(e) => {
                println!("Chan error {:?}", e);
                return e;
            }
        };

        loop {
            match manage_ui() {
                UIResult::Play => {
                    println!("play");
                    chan.set_paused(false);
                }
                UIResult::Pause => {
                    println!("pause");
                    chan.set_paused(true);
                }
                UIResult::Exit => {
                    println!("exit");
                    chan.stop();
                    break;
                }
                UIResult::Error => {
                    println!("error");
                    chan.stop();
                    break;
                }
                UIResult::NA => {}
            }
        }
    }

    return rfmod::Result::Ok;
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

    songs = vec![String::from("/home/nathan/Downloads/left.mp3"),
                 String::from("/home/nathan/right.mp3")];
    let mut playlist = playlist::Playlist::new(songs);

    let rfmod = match init_fmod() {
        Ok(f) => f,
        Err(e) => {
            println!("rfmod init error: {:?}", e);
            return;
        }
    };

    println!("Commands:");
    println!("\tPlay : l");
    println!("\tPause: p");
    println!("\tExit : x");

    match std::fs::metadata("/home/nathan/Downloads/right.mp3") {
        Ok(a) => println!("looks good"),
        Err(e) => println!("Error: {}", e),
    }

    main_loop(&rfmod, &mut playlist);
}
