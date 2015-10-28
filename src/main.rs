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

extern crate sdl2;
extern crate sdl2_mixer;
extern crate argparse;

mod playlist;

use std::io;
use argparse::{ArgumentParser, Store, List};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2_mixer::{INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG,
                 INIT_OGG, DEFAULT_FREQUENCY};

enum UIResult {
    Play,
    Pause,
    Exit,
    Error,
    NA,
}

fn init_fmod() {
    let sdl_context = sdl2::init().unwrap();
    let _audio = sdl_context.audio().unwrap();

    println!("mixer initialized: {}", sdl2_mixer::init(
            INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
            INIT_MODPLUG | INIT_OGG).bits());

    let _ = sdl2_mixer::open_audio(DEFAULT_FREQUENCY, 0x8010u16, 2, 1024);
    sdl2_mixer::allocate_channels(0);

    {
        let n = sdl2_mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in (0..n) {
            println!("  decoder {} => {}", i, sdl2_mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2_mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in (0..n) {
            println!("  decoder {} => {}", i, sdl2_mixer::get_music_decoder(i));
        }
    }

    println!("query spec => {:?}", sdl2_mixer::query_spec());
}

//fn manage_ui() -> UIResult {
    //let stdin = io::stdin();

    //let mut input = String::new();
    //let read;

    //read = stdin.read_line(&mut input);
    //if read.is_err() {
        //return UIResult::Error;
    //}

    //match &*input {
        //"l\n" => UIResult::Play,
        //"p\n" => UIResult::Pause,
        //"x\n" => UIResult::Exit,
        //_ => {
            //println!("Unknown Command");
            //UIResult::NA
        //}
    //}
//}

//fn main_loop(rfmod: &rfmod::Sys, playlist: &mut playlist::Playlist) -> rfmod::Result {
    //loop {
        //let song = match playlist.get_next_song() {
            //Some(a) => String::from("/home/nathan/Downloads/right.mp3"),
            //None => break,
        //};
        //println!("{}", song);
        //let sound = match rfmod.create_sound(&song, None, None) {
            //Ok(f) => f,
            //Err(e) => {
                //println!("Make sound error {:?}", e);
                //return e;
            //}
        //};

        //let chan = match sound.play() {
            //Ok(f) => f,
            //Err(e) => {
                //println!("Chan error {:?}", e);
                //return e;
            //}
        //};

        //loop {
            //match manage_ui() {
                //UIResult::Play => {
                    //println!("play");
                    //chan.set_paused(false);
                //}
                //UIResult::Pause => {
                    //println!("pause");
                    //chan.set_paused(true);
                //}
                //UIResult::Exit => {
                    //println!("exit");
                    //chan.stop();
                    //break;
                //}
                //UIResult::Error => {
                    //println!("error");
                    //chan.stop();
                    //break;
                //}
                //UIResult::NA => {}
            //}
        //}
    //}

    //return rfmod::Result::Ok;
//}

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

    let filename = std::path::Path::new("/home/nathan/Music/Brite Futures/Glistening Pleasure 2.0/04 - Iceage Babeland.mp3");

    let audio_subsystem = init_fmod();

    // We want music to be freed before we quit sdl2_mixer (otherwise an error
    // happens), so we do all of the following within its own block.
    {
        let music = sdl2_mixer::Music::from_file(filename).unwrap();

        fn hook_finished() {
            println!("play ends! from rust cb");
        }

        sdl2_mixer::Music::hook_finished(hook_finished);

        println!("music => {:?}", music);
        println!("music type => {:?}", music.get_type());
        println!("music volume => {:?}", sdl2_mixer::Music::get_volume());
        println!("play => {:?}", music.play(1));

        std::thread::sleep_ms(10000);

        sdl2_mixer::Music::halt();
    }

    println!("Commands:");
    println!("\tPlay : l");
    println!("\tPause: p");
    println!("\tExit : x");

    //main_loop(&rfmod, &mut playlist);
}
