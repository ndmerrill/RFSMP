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
use std::sync::mpsc as cc;
pub struct Playlist {
    pub songs: Vec<String>,
    pub song_index: i32,
    sender: cc::Sender<i32>,
}

impl Playlist {
    pub fn new(songs: Vec<String>, sender: cc::Sender<i32>) -> Playlist {
        Playlist{songs: songs,
                 song_index: -1,
                 sender: sender}
    }

    // Returns the next song that should be played.
    pub fn get_next_song(&mut self) -> Option<&str> {
        match self.songs.get((self.song_index+1) as usize) {
            Some(a) => Some(&a),
            None => None,
        }
    }

    // Tells playlist that the current song is done and to proceed to the
    // next one.
    pub fn go_to_next(&mut self) {
        self.song_index += 1;
        self.sender.send(self.song_index);
    }

    // Tells playlist to go back to the song before.
    pub fn go_to_prev(&mut self) {
        self.song_index -= 1;
        self.sender.send(self.song_index);
    }

    // Returns the song that should currently be playing
    pub fn get_curr_song(&self) -> Option<&str> {
        match self.songs.get(self.song_index as usize) {
            Some(a) => Some(&a),
            None => None,
        }
    }
}
