//  Copyright 2015 Nathanael Merrill

//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at

//    http://www.apache.org/licenses/LICENSE-2.0

//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

extern crate rfmod;

mod playlist;

use std::io;

fn init_fmod() -> Result<rfmod::Sys, rfmod::Result> {
    let fmod = try!(rfmod::Sys::new());

    match fmod.init() {
        rfmod::Result::Ok => Ok(fmod),
        e => Err(e),
    }
}

fn make_channel(rfmod: &rfmod::Sys, song: &str) -> Result<rfmod::Sound, rfmod::Result> {
    rfmod.create_sound(song, None, None)
}

fn main() {
    let rfmod = match init_fmod() {
        Ok(f) => f,
        Err(e) => {
            println!("rfmod init error: {:?}", e);
            return;
        }
    };

    let playlist = playlist::Playlist::new("/home/nathan/Downloads/left.mp3");

    let songa = match make_channel(&rfmod, playlist.get_next_song()) {
        Ok(f) => f,
        Err(e) => {
            println!("Make sound error {:?}", e);
            return;
        }
    };

    let song = match songa.play() {
        Ok(f) => f,
        Err(e) => {
            println!("Chan error {:?}", e);
            return;
        }
    };

    match song.is_playing() {
        Ok(f) => println!("{}", f),
        Err(e) => println!("{:?}", e),
    }

    let stdin = io::stdin();

    let mut input = String::new();
    let mut read;

    loop {
        // Make your choice
        println!("Commands:");
        println!("\tPlay : l");
        println!("\tPause: p");
        println!("\tExit : x");

        input.clear();
        read = stdin.read_line(&mut input);
        if read.is_err() {
            println!("Error: {}", read.unwrap_err());
            return;
        }

        match &*input {
            "l\n" => {
                println!("play");
                song.set_paused(false);
            }
            "p\n" => {
                println!("pause");
                song.set_paused(true);
            }
            "x\n" => {
                println!("exit");
                song.stop();
                break;
            }
            _  => println!("Unknwon command."),
        }
    }
    println!("Goodbye!");
}
