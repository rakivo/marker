#![feature(let_chains)]
#[allow(unused_imports)]
use std::io::{
    prelude::*,
    BufReader,
    BufWriter
};
use std::fs::File;

// TODO:
// States for md. attrs.

macro_rules! read_buf {
    ($buf: ident <- $rbuf: ident) => {
        $rbuf.read_line(&mut $buf).ok();
        let $buf = $buf.trim().to_owned();
    };
}

macro_rules! fwrite_line {
    ($wbuf: ident <- $input: ident) => {
        $input.iter().for_each(|inp| $wbuf.write_all(format!("{inp}\n").as_bytes()).expect("Failed to write"));
    };
}

macro_rules! hselect_state {
    ($h_state: ident, $h: ident) => {
        $h_state.select_state(&($h.to_digit(10).unwrap_or(0) as usize - 1))
    }
}

#[derive(Debug)]
// NOC -> not overcomplicated
enum NOCError {
    IndexOutOfBounds
}

trait State {
    fn off_all(&mut self);
    fn if_any_on(&self) -> Option<usize>;
}

// CState -> Code state (in-``` mode)
struct CState {
    active:     bool,
    multi_line: bool
}

impl CState {
    pub fn new() -> CState {
        CState {
            active:     false,
            multi_line: false
        }
    }

    fn on(&mut self, m: &bool) {
        self.active = true;
        self.multi_line = *m;
    }
}

impl State for CState {
    fn off_all(&mut self) {
        self.active     = false;
        self.multi_line = false;
    }

    // 0 -> active, 1 -> multi_line
    fn if_any_on(&self) -> Option<usize> {
        if self.active {
            return Some(0)
        } else if self.multi_line {
            return Some(1)
        } None
    }
}

// HState -> header state (# things)
struct HState {
    hs: [bool; 6]
}

impl HState {
    pub fn new() -> HState {
        HState {
            hs: [false; 6]
        }
    }

    fn select_state(&mut self, which: &usize) -> Result<(), NOCError> {
        let which = *which;
        if which < 6 {
            self.hs[which] = true;
            self.off_other(&which);
            Ok(())
        } else {
            Err(NOCError::IndexOutOfBounds)
        }
    }

    fn off_other(&mut self, which: &usize) {
        for (i, h) in self.hs.iter_mut().enumerate() {
            if i != *which {
                *h = false
            }
        }
    }
}

impl State for HState {
    fn off_all(&mut self) {
        self.hs.iter_mut().for_each(|h| *h = false);
    }

    fn if_any_on(&self) -> Option<usize> {
        for (i, &h) in self.hs.iter().enumerate() {
            if h { return Some(i); }
        } None
    }
}

// h: in h state, d: in def state
fn input_loop(mut h_state: HState, mut c_state: CState, mut input: Vec<String>, f: File) -> std::io::Result<()> {
    let mut wbuf = BufWriter::new(f);
    let mut rbuf = BufReader::new(std::io::stdin().lock());
    let mut buf  = String::new();

    let mut code_input = Vec::new();

    loop {
        h_state.off_all();
        buf.clear();
        read_buf!(buf <- rbuf);

        if buf == "q" { break; }
        // if there's any mode-symbols at the start
        if let Some(first) = buf.chars().nth(0)
        {
        match first {
            '#' => if let Some(h) = buf.chars().nth(1) {
                if h.is_digit(10) {
                    match hselect_state!(h_state, h) {
                        Err(e) => {
                            eprintln!("ERROR: {e:?}");
                            continue;
                        }
                        Ok(_)  => {}
                    }
                } else {
                    h_state.select_state(&0).ok(); // interpret # without number after it as single #
                }
            }
            '`' => {
                let n = buf.len();
                if let Some(last) = buf.chars().nth(n - 1) {
                    if last.eq(&'`') {
                        c_state.on(&true);
                        let code = &buf[1..n - 2];
                        println!("c: {code}");
                        code_input.push(code.to_owned());
                        continue;
                    } else {
                    }
                }
            }
            _  => {}
        }
        }
        if let Some(h) = h_state.if_any_on() {
            let inp = format!("{h} {input}",
                h = (0..=h).map(|_| "#").collect::<String>(),
                input = &buf[1..].split_whitespace().collect::<Vec<_>>().join(" "));
            println!("h: {inp}");
            input.push(inp);
        }
    }
    fwrite_line!(wbuf <- input);
    Ok(())
}

const FILE_NAME: &str = "test";
fn main() -> std::io::Result<()> {
    let h_state = HState::new();
    let c_state = CState::new();
    let input   = Vec::new();
    let f       = File::create(&format!("{FILE_NAME}.md"))?;
    input_loop(h_state, c_state, input, f)
}
