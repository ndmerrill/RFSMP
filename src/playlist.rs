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

pub struct Playlist {
    songs: Vec<String>,
    song_index: i32,
}

impl Playlist {
    pub fn new(songs: Vec<String>) -> Playlist {
        Playlist{songs: songs,
                 song_index: -1}
    }

    pub fn get_next_song(&mut self) -> Option<&str> {
        self.song_index += 1;
        match self.songs.get(self.song_index as usize) {
            Some(a) => Some(&a),
            None => None,
        }
    }
    
    pub fn get_prev_song(&mut self) -> Option<&str> {
        self.song_index += -1;
        match self.songs.get(self.song_index as usize) {
            Some(a) => Some(&a),
            None => None,
        }
    }
}
