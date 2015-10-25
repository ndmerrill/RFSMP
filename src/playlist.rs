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

use std::string;

pub struct Playlist {
    music: string::String,
}

impl Playlist {
    pub fn new(song: &str) -> Playlist {
        Playlist{music: song.to_string()}
    }

    pub fn get_next_song(&self) -> &str {
        &self.music
    }
}
