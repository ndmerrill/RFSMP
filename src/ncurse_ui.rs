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


//uifeature:
// press / to start filter/seatch
// change playlist to only display the things using regex filter of vector
// refresh/serach only when ch thread stops hanging for maximum performance
use playlist;
use ncurses::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc as cc;
use std::thread;
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
    song_index: i32,
    y: i32,
}
//there are three threads, one that hangs on the keyinput (the ncurse one), one that hangs on retrieving the data from main(which prints the output), and the gstreamer loop
//the cool part about this is that there is no looping in the ui, only hanging threads that send messages to awake other hanging threads

//current bug is in a stack thread in mobile bookmarks folder
impl UI {

    pub fn new(tx :cc::Sender<UIResult>, recvr :cc::Receiver<i32>, pl :Vec<String>) -> UI {
        initscr();
        noecho();
        refresh();
        start_color();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        init_pair(1, COLOR_RED, COLOR_BLACK);
        init_pair(2, COLOR_GREEN, COLOR_BLACK);
        print_songs(&pl, 0);
        let mut te = Arc::new(Mutex::new(tx.clone()));
        thread::Builder::new().name("ncursehanger".to_string()).spawn(move || {
            while true {
                let mut ry :i32 = 0;
                let mut rx :i32 = 0;
                getmaxyx(stdscr, &mut ry, &mut rx);
                // printw(&*string::from(rx.to_string() + " , " + &ry.to_string()));
                let mut ch = getch();
                // printw(&*ch.to_string());
                match ch {
                    //n
                    110 => {te.lock().unwrap().send(UIResult::Next);},
                    //p
                    112 => {te.lock().unwrap().send(UIResult::Previous);},
                    //x
                    120 => {te.lock().unwrap().send(UIResult::Exit);},
                    //spc
                    32 => {te.lock().unwrap().send(UIResult::PlayPause);},
                    _ => {printw("no binding");}
                }
            };
        });
        while true {
            let a = recvr.recv();
            match a {
                Ok(x) => {print_songs(&pl, x);},
                Err(_) => {tx.send(UIResult::Exit);}
            }
        };
        return UI {
            song_index: 0,
            y: 3,
        };
    }

}

//we should have shared state here..... even though i just spent 1hr avoiding it.
//if we dont, its rediculouse rewritten code.
//try to make it shared state with arcs/mutex but dont put the arc::new inside the thread clojure u dummy
//make the origional initiaition of playlist an arc, so that the gst part is also using arc
fn print_songs(pl: &Vec<String>, indexa: i32) {
    for (x , i) in pl.iter().zip(0..) {
        match (indexa == i) {
            false => {},
            true => {attron(COLOR_PAIR(1));},
        };
        mvprintw(i, 0, x);
        attroff(COLOR_PAIR(1));
        attron(COLOR_PAIR(2));
    }
    refresh();
}
