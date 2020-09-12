use clap::{App, Arg, ArgMatches};
use minifb::{Window, WindowOptions};
use std::thread;
use std::time;
use std::vec::Vec;

static ALIVE: u32 = 0x42069;
static DEAD: u32 = 0;

struct GollyBuff {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
    win: Window,
}

fn oflow(input: i32, max: usize) -> usize {
    let input = input % max as i32;
    if input < 0 {
        (max as i32 + input) as usize
    } else {
        input as usize
    }
}

impl GollyBuff {
    fn new(name: &str, width: usize, height: usize, scale: minifb::Scale) -> GollyBuff {
        let mut opt = WindowOptions::default();
        opt.scale = scale;
        GollyBuff {
            width,
            height,
            pixels: vec![0x000000; width * height],
            win: Window::new(name, width, height, opt).unwrap(),
        }
    }

    fn xy_to_pos(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x - 1
    }

    fn update(&mut self) {
        self.win
            .update_with_buffer(&self.pixels, self.width, self.height)
            .unwrap();
    }

    fn move_index(&self, anchor: usize, horiz: i32, vert: i32) -> usize {
        let current_row = anchor / self.width;
        let new_row = oflow(current_row as i32 + vert, self.height);
        let new_column = oflow(horiz + anchor as i32, self.width);
        (self.width * new_row) + new_column
    }

    fn neighbors(&self, index: usize) -> [usize; 8] {
        let mut arr = [0usize; 8];
        let anchor_row = index / self.width;
        let wrap = |x: i32, row: i32| {
            let row = oflow(anchor_row as i32 + row, self.height);
            let row_index = row * self.width;
            oflow(x - row_index as i32, self.width) + row_index
        };

        let index = index as i32;
        for i in 0..=2 {
            arr[i] = wrap(index + i as i32 - 1, -1);
        }
        arr[3] = wrap(index - 1, 0);
        arr[4] = wrap(index + 1, 0);
        let mut loc = 5_usize;
        for i in 0..=2 {
            arr[loc] = wrap(index + i - 1, 1);
            loc += 1;
        }
        arr
    }

    fn live_neighbors(&self, index: usize) -> u8 {
        let mut ctr = 0;
        for i in self.neighbors(index).iter() {
            if self.pixels[*i] != 0 {
                ctr += 1;
            }
        }
        ctr
    }

    fn all_live_neighbors(&self) -> Vec<u8> {
        let mut neighbors: Vec<u8> = Vec::with_capacity(self.pixels.len());
        for i in 0..self.pixels.len() {
            neighbors.push(self.live_neighbors(i));
        }
        neighbors
    }

    fn classic_life(&mut self, survive_rule: &Vec<u8>, born_rule: &Vec<u8>) {
        let neighborvec = self.all_live_neighbors();
        for (value, neighbors) in self.pixels.iter_mut().zip(neighborvec) {
            if *value == 0x000000 {
                if born_rule.contains(&neighbors) {
                    *value = ALIVE;
                }
            } else {
                if !survive_rule.contains(&neighbors) {
                    *value = 0x000000;
                }
            }
        }
    }

    fn elementary_parents(&self, index: usize) -> u8 {
        let mut parents = 0u8;
        for i in 0..=2 {
            parents |= ((self.move_index(index, 1 - i, -1) != 0) as u8) << i;
        }
        parents
    }

    fn elementary(&mut self, rule: u8) {
        let mut new_cells: Vec<usize> = Vec::new();
        for index in 0..self.pixels.len() {
            let mut parents = 0u8;
            for i in 0..=2 {
                parents |= ((self.move_index(index, 1 - i, -1) != 0) as u8) << i;
            }
            if rule & (1 << parents) != 0 {
                new_cells.push(index);
            }
        }
        for cell in new_cells {
            self.pixels[cell] = ALIVE;
        }
    }

    fn clear(&mut self) {
        self.pixels = vec![0x000000; self.pixels.len()];
    }
}

fn parse_rule(rule: &str) -> (Vec<u8>, Vec<u8>) {
    let mut survive: Vec<u8> = Vec::new();
    let mut born: Vec<u8> = Vec::new();

    let mut rule = rule.chars();
    for num in rule.by_ref().take_while(|x| *x != '/') {
        survive.push(num.to_digit(10).unwrap() as u8);
    }
    for num in rule {
        born.push(num.to_digit(10).unwrap() as u8);
    }
    (survive, born)
}

fn main() {
    let (width, height) = (350, 200);
    let mut buff = GollyBuff::new("yuh", width, height, minifb::Scale::X4);

    let day_night_survive: Vec<u8> = vec![2, 3, 5];
    let day_night_born: Vec<u8> = vec![3, 5];

    let matches = App::new("Ian's game of life")
        .args_from_usage("-r --rule=[RULE] 'Sets the rule to be used. Default is 23/3'")
        .get_matches();

    let rule = match matches.value_of("rule") {
        Some(string) => parse_rule(string),
        None => (vec![2, 3], vec![3]),
    };

    /*
    for i in buff.neighbors(1300).iter() {
        buff.pixels[*i] = WHITE;
    }

    let glider = 15100;

    for i in 0..=2 {
        buff.pixels[glider + i] = WHITE;
    }

    buff.pixels[glider + buff.width + 2] = WHITE;
    buff.pixels[glider + (buff.width * 2) + 1] = WHITE;
    buff.pixels[glider + (buff.width * 2) + 2] = WHITE;

    buff.update();
    */

    buff.win
        .limit_update_rate(Some(time::Duration::from_millis(30)));
    let mut paused = true;
    let mode = |buff: &mut GollyBuff| buff.classic_life(&rule.0, &rule.1);
    loop {
        if buff.win.get_mouse_down(minifb::MouseButton::Left) {
            if let Some((x, y)) = buff.win.get_mouse_pos(minifb::MouseMode::Discard) {
                let pos = buff.xy_to_pos(x as usize, y as usize);
                buff.pixels[pos] = ALIVE;
            }
        }
        if buff.win.is_key_released(minifb::Key::Space) {
            paused = !paused;
        }
        if !paused {
            //buff.classic_life(&day_night_survive, &day_night_born);
            mode(&mut buff);
        }
        if buff
            .win
            .is_key_pressed(minifb::Key::R, minifb::KeyRepeat::No)
        {
            buff.clear();
        }
        if buff
            .win
            .is_key_pressed(minifb::Key::Escape, minifb::KeyRepeat::No)
        {
            break;
        }
        if paused
            && buff
                .win
                .is_key_pressed(minifb::Key::Period, minifb::KeyRepeat::Yes)
        {
            mode(&mut buff);
        }
        buff.update();
    }
}
