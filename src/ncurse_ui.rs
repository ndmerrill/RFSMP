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

use playlist;
use ncurses::*;

use std::sync::mpsc as cc;
use std::error::Error;
pub enum UIResult {
    PlayPause,
    Next,
    Previous,
    Exit,
    Error(String),
    NA,
}

pub struct UI {
    x: i32,
    y: i32,
}
//next step is to make the ui listing songs, and the step after that is to make the other window (rendered in main.rs)
// that renders the timing progress bar based on the play loop
impl UI {

    pub fn new(tx :cc::Sender<UIResult>) -> UI {
        initscr();
        refresh();
        let mut rx :i32 = 0;
        let mut ry :i32 = 0;
        while true {
            getmaxyx(stdscr, &mut ry, &mut rx);
            // printw(&*String::from(rx.to_string() + " , " + &ry.to_string()));
            let mut ch = getch();
            // printw(&*ch.to_string()); -- good for finding keycodes
            match ch {
                //n
                110 => {tx.send(UIResult::Next);},
                //p
                112 => {tx.send(UIResult::Previous);},
                //x
                120 => {tx.send(UIResult::Exit);},
                //spc
                32 => {tx.send(UIResult::PlayPause);},
                _ => {printw("no binding");}
            }
            refresh();
        }
        return UI {
            x: 10,
            y: 3,
        };
    }
}
