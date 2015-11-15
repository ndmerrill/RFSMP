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

use std::io;
use rustty::{
    Terminal,
    Event,
    HasSize,
    CellAccessor,
    Cell,
    Attr,
    Color,
};
use rustty::ui::{
    Painter,
    Dialog,
    Widget,
    Alignable,
    HorizontalAlign,
    VerticalAlign
};

fn create_optiondlg(length: usize) -> Dialog {
    let mut optiondlg = Dialog::new(length as usize, 4);

    let inc_label = "space --> play/pause";
    let q_label   = "x     --> exit program";
    let n_label   = "n --> next song";
    let pp_label  = "p --> prev song";

    let inc_pos = optiondlg.window().halign_line(inc_label, HorizontalAlign::Left, 1);
    let q_pos = optiondlg.window().halign_line(q_label, HorizontalAlign::Left, 1);
    let n_pos = optiondlg.window().halign_line(n_label, HorizontalAlign::Middle, 1);
    let pp_pos = optiondlg.window().halign_line(pp_label, HorizontalAlign::Middle, 1);

    optiondlg.window_mut().printline(inc_pos, 1, inc_label);
    optiondlg.window_mut().printline(q_pos, 2, q_label);
    optiondlg.window_mut().printline(n_pos, 1, n_label);
    optiondlg.window_mut().printline(pp_pos, 2, pp_label);
    optiondlg.window_mut().draw_box();
    optiondlg
}

pub enum UIResult {
    Play,
    Pause,
    Next,
    Exit,
    Error,
    PlayPause,
    Previous,
    NA,
}

pub struct UI {
    stdin: io::Stdin,
    term: Terminal,
    optiondlg: Dialog,
    canvas: Widget,
    list_canvas: Widget,
    length: usize,
    height: usize,
    songs: Vec<String>,
}
//TODO, convert incoming strings from path to cool string
impl UI {
    pub fn new(songs: Vec<String>) -> UI {
        // TODO this is where i should convert the vec<Path> to string
        // by running the method path_to_string on ever single one. 
        // (implement path_to_string manually)
        //  but only once nathan implements Paths
        // Create our terminal, dialog window and main canvasa
        let curr_song = "";
        let mut term = Terminal::new().unwrap();
        let length = term.cols();
        let height = term.rows();
        // aligns everything
        let mut optiondlg = create_optiondlg(length);
        let mut canvas = Widget::new(length as usize, 2);
        let mut list_canvas = Widget::new(length as usize, songs.len());
        optiondlg
            .window_mut()
            .align(&term, HorizontalAlign::Middle, VerticalAlign::Bottom, 0);
        canvas.align(&term, HorizontalAlign::Middle, VerticalAlign::Bottom, 4);
        list_canvas.align(&term, HorizontalAlign::Middle, VerticalAlign::Top, 0);

        UI {stdin: io::stdin(),
            term: term,
            optiondlg: optiondlg,
            canvas: canvas,
            list_canvas: list_canvas,
            length: length,
            height: height,
            songs: songs,
        }
    }

    fn length_checker(&mut self) {
        let last_length = self.length;
        let last_pos = (self.length, self.height);
        self.length = self.term.cols();
        self.height =  self.term.rows();
        match last_pos == (self.term.cols(), self.term.rows()){
            true => {},
            false => {
                self.term.clear();
                self.optiondlg = create_optiondlg(self.length);
                self.canvas = Widget::new(self.length as usize, 2);
                self.optiondlg
                    .window_mut()
                    .align(&self.term,
                           HorizontalAlign::Middle,
                           VerticalAlign::Bottom, 0);
                self.canvas.align(&self.term,
                                  HorizontalAlign::Middle,
                                  VerticalAlign::Bottom, 6);
                self.list_canvas.align(&self.term, HorizontalAlign::Left,
                                       VerticalAlign::Top, 0);
            },
        }

    }

    pub fn manage_ui(&mut self, playlist: &playlist::Playlist,
                     time: i32, totaltime: i32) -> UIResult {
        //TODO: The rest of this function won't run if there is input
        //this is not really ideal
        while let Some(Event::Key(ch)) = self.term.get_event(0).unwrap() {
            match ch {
                ' ' => return UIResult::PlayPause,
                'p' => return UIResult::Previous,
                'n' => return UIResult::Next,
                'x' => return UIResult::Error,
                _  => return UIResult::NA,
            }
        }

        let curr_song = playlist.get_curr_song().unwrap();
        let length_i32 = self.length as i32;
        let tnum = time.to_string().len() + curr_song.len() + totaltime.to_string().len();
        let mut num = tnum as i32;
        num = length_i32 - num - 8;
        num = num / 2;
        let mut append = vec![' '; num as usize].into_iter().collect::<String>();
        let append2 = match length_i32.wrapping_rem(2) {
            0 => append.clone(),
            1 => append.clone() + " ",
            _ => unreachable!(),
        };
        let display = format!("--{}{}--{}--{}{}--", time, append, 
                              curr_song, append2, totaltime);

        let v: Vec<char> = display.chars().collect();
        let (cols, rows) = self.canvas.size();
        let (cols, rows) = (cols as isize, rows as isize);
        let mut num_x = 8.0;
        let mut num_not = 4.0;

        if totaltime == 0 { //TODO this is bad. fix it
            num_x = 0.0;
        }
        else {
            num_x = time as f32 / totaltime as f32;
        }
        num_x = num_x * length_i32 as f32;
        num_not = length_i32 as f32 - num_x as f32;

        let mut va = vec!['x'; num_x.round() as usize];
        let mut ev = vec!['-'; num_not.round() as usize];
        for x in ev {
            va.push(x);
        }
        //v.append(&mut va); unstable
        for i in 0..cols*rows {
            let y = i as isize / cols;
            let x = i as isize % cols;
            let fep ='*';
            let mut cell = self.canvas.get_mut(x as usize, y as usize).unwrap();
            match y {
                0 => cell.set_ch(*v.get(x as usize).unwrap_or_else(|| &fep)),
                1 => cell.set_ch(*va.get(x as usize).unwrap_or_else(|| &fep)),
                _ => cell.set_ch(' '),
            };
        }

        self.length_checker();
        self.second_panel(curr_song);
        self.canvas.draw_into(&mut self.term);
        self.list_canvas.draw_into(&mut self.term);
        self.optiondlg.window().draw_into(&mut self.term);
        self.term.swap_buffers().unwrap();
        return UIResult::NA;
    }

    fn second_panel(&mut self, curr_song: &str) {
        let cell = Cell::with_style(Color::Black, Color::Red, Attr::Default);
        let cellother = Cell::with_style(Color::Default, Color::Default, Attr::Default);
        let (cols, rows) = self.list_canvas.size();
        let mut counter = 0;
        let fep = "*".to_string();
        //println!("{}", self.songs.get(counter).unwrap_or_else(|| &fep));

        for i in 0..rows {
            let song = self.songs.get(counter).unwrap_or_else(|| &fep);
            self.list_canvas
                .printline_with_cell(0, counter as usize, song, match &curr_song == song {
                    true => cell,
                    false => cellother,
                });
            counter = counter + 1;
        }
    }
}
