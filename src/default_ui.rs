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

use std::io;

pub enum UIResult {
    Play,
    Pause,
    Next,
    Exit,
    Error,
    NA,
}

pub struct UI {
    stdin: io::Stdin,
}

impl UI {
    pub fn new() -> UI {
        println!("Commands:");
        println!("\tPlay : l");
        println!("\tPause: p");
        println!("\tNext: n");
        println!("\tExit : x");

        UI {stdin: io::stdin()}
    }

    pub fn manage_ui(&self) -> UIResult {
        let mut input = String::new();
        let read;

        read = self.stdin.read_line(&mut input);
        if read.is_err() {
            return UIResult::Error;
        }

        match &*input {
            "l\n" => UIResult::Play,
            "p\n" => UIResult::Pause,
            "x\n" => UIResult::Exit,
            "n\n" => UIResult::Next,
            _ => {
                println!("Unknown Command");
                UIResult::NA
            }
        }
    }
}
